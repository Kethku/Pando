use std::{any::Any, cell::RefCell, collections::{HashMap, HashSet}, sync::Arc};

use vello::{kurbo::{Affine, Size}, Scene};

use crate::{
    context_stack::{
        AttachedContext, Context, ContextEventLoop, ContextWindow, DrawContext, EventState, LayoutContext, UpdateContext
    },
    element::{Element, ElementPointer},
    mouse_region::MouseRegionManager,
    shaper::Shaper,
    token::Token,
};

pub struct Application<Root: Element> {
    pub mouse_region_manager: RefCell<MouseRegionManager>,
    pub event_state: EventState,
    pub shaper: RefCell<Shaper>,

    pub regions: RefCell<HashMap<Token, (Affine, Size)>>,
    pub states: RefCell<HashMap<Token, Box<dyn Any>>>,
    pub focused_element: RefCell<Option<Token>>,
    pub base_token: Token,
    pub force_redraw: bool,

    pub root: RefCell<ElementPointer<Root>>,
    pub tokens: Arc<Vec<Token>>,
}

impl<Root: Element> Application<Root> {
    pub fn new<F>(root_constructor: F) -> Self
        where F: for<'a> FnOnce(&mut Context<'a>) -> ElementPointer<Root>,
    {
        let event_state = EventState::new();
        let shaper = RefCell::new(Shaper::new());
        let states = RefCell::new(HashMap::new());
        let focused_element = RefCell::new(None);
        let base_token = Token::new::<Self>();

        let root = {
            let tokens = Vec::new();
            let mut context = Context::new(&event_state, &shaper, &states, &focused_element, base_token, &tokens);
            RefCell::new(root_constructor(&mut context))
        };

        Application {
            mouse_region_manager: RefCell::new(MouseRegionManager::new()),
            event_state,
            shaper,

            regions: RefCell::new(HashMap::new()),
            states,
            tokens: Arc::new(Vec::new()),
            focused_element,
            base_token,
            force_redraw: false,

            root
        }
    }

    pub fn token(&self) -> Token {
        self.root.borrow().token()
    }

    pub fn tick<'a>(&mut self, window: &'a dyn ContextWindow, event_loop: &'a dyn ContextEventLoop) -> Option<Scene> {
        self.refresh_tokens();
        let mut redraw_requested = self.process_mouse_regions(window, event_loop);
        redraw_requested |= self.update(window, event_loop);

        let drawn_scene = if redraw_requested || self.force_redraw {
            let child_lookup = self.layout(window, event_loop);
            let scene = self.draw(child_lookup, window, event_loop);

            Some(scene)
        } else {
            None
        };

        self.event_state.next_frame();

        drawn_scene
    }

    pub fn refresh_tokens(&mut self) {
        let root = self.root.borrow();
        self.tokens = Arc::new(root.tokens());
    }

    pub fn context<'a>(&'a self) -> Context<'a> {
        Context::new(
            &self.event_state,
            &self.shaper,
            &self.states,
            &self.focused_element,
            self.base_token, 
            &self.tokens
        )
    }

    pub fn attached_context<'a>(&'a self, window: &'a dyn ContextWindow, event_loop: &'a dyn ContextEventLoop) -> AttachedContext<'a> {
        AttachedContext::new(
            Context::new(
                &self.event_state,
                &self.shaper,
                &self.states,
                &self.focused_element,
                self.base_token, 
                &self.tokens
            ),
            window,
            event_loop
        )
    }

    pub fn process_mouse_regions(&self, window: &dyn ContextWindow, event_loop: &dyn ContextEventLoop) -> bool {
        let mut mouse_region_manager = self.mouse_region_manager.borrow_mut();
        let mut regions = self.regions.borrow_mut();
        mouse_region_manager.process_regions(&mut regions, self.attached_context(window, event_loop))
    }

    pub fn update(&self, window: &dyn ContextWindow, event_loop: &dyn ContextEventLoop) -> bool {
        let mut mouse_region_manager = self.mouse_region_manager.borrow_mut();
        let mut redraw_requested = false;
        let mut update_context = UpdateContext::new(
            self.attached_context(window, event_loop),
            &mut mouse_region_manager,
            &mut redraw_requested,
        );

        let mut root = self.root.borrow_mut();
        root.update(&mut update_context);
        redraw_requested
    }

    pub fn layout(&self, window: &dyn ContextWindow, event_loop: &dyn ContextEventLoop) -> HashMap<Token, HashSet<Token>> {
        let mut regions = self.regions.borrow_mut();
        let mut child_lookup = HashMap::new();
        {
            let mut layout_context = LayoutContext::new(
                self.attached_context(window, event_loop),
                &mut regions,
                &mut child_lookup,
            );
            let mut root = self.root.borrow_mut();
            let result = root.layout(
                self.event_state.window_size,
                self.event_state.window_size,
                &mut layout_context,
            );
            result.position(Affine::IDENTITY, &mut layout_context);
        }

        child_lookup
    }

    pub fn draw<'a>(&mut self, child_lookup: HashMap<Token, HashSet<Token>>, window: &dyn ContextWindow, event_loop: &dyn ContextEventLoop) -> Scene {
        let mut mouse_region_manager = self.mouse_region_manager.borrow_mut();
        let regions = self.regions.borrow();
        let root = self.root.borrow();

        mouse_region_manager.clear_regions();
        let mut scene = Scene::new();
        let mut draw_context = {
            let context = self.attached_context(window, event_loop);
            DrawContext::new(context, &mut mouse_region_manager, &child_lookup, &regions, &mut scene)
        };
        root.draw(&mut draw_context);
        // mouse_region_manager.draw_mouse_regions(&mut scene);
        self.force_redraw = false;

        scene
    }

    pub fn with_root<Result>(&self, callback: impl FnOnce(&Root, &Context) -> Result) -> Result {
        let root = self.root.borrow();
        root.with_context(&self.context(), |cx| {
            callback(&root, cx)
        })
    }
}
