#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
// mod ngs;
mod todo;
mod util;

// A NodeGraph or .ng file format and editor for node graph data.
// Backing format is a sqlite database which contains:
//  - a specification for the node data it contains (or a pointer to a file that does) authored
//    as a node graph itself defined in a canonical format called NodeGraphSpecification or
//    NGS
//  - the node graph itself governed by the above specification
//  - non functional data such as node positions, zoom level, organizational tools such as groups
//  - optionally a complete history of the changes to the graph
//
// NGS specifies the following information:
//  - the structure of the data that gets extracted
//  - how the data should be authored and displayed
//
// Parsing libraries can generate types and functions for modifying and extracting data from the
// file format. This is done by parsing the NGS and generating types to extract into and functions
// for reading those types from the database
//
// The editor is an infinite 2d canvas with zoom and pan and standardized node authoring tools. It
// should combine the best experiences from existing node systems such as Unreal Engine, and
// Blender while looking to innovate in the space.
//
// Initial Applications for this file format include:
//  - NGS itself. Authoring of node graph specifications is done in Pando itself
//  - Todo tree application. NGS should be expressive enough to create a full functioning Todo Tree
//    app entirely in it and easily
//  - Graphics Pipeline Declaration. NG files are supposed to introduce visual programming and
//    authorship to existing applications. If a problem is visual in nature, adding an NG file and
//    a parser for the output information should be a natural fit. Graph pipelines are a good
//    example where I see this helping
//  - Shader/VFX node graphs. This is a no brainer as shaders and vfx are the most developed
//    application of node graphs today. This usecase will likely motivate plugins to hook into the
//    editor in order to provide live previous from an external source
//  - Simple scripting languages. This would push what is currently possible for the system in
//    important ways. Ideally doing this well would improve authoring for all usecases
//
// TODO LIST UI
//
// - Connect points snap to corners, box middles, and existing connect points
// - On drag, show a radial menu with line types
//   - If no direction is chosen, cancel the connection
//   - Once a direction is chosen, the line starts drawing
//   - Each option is rendered as a path cut off slice of the circle centered
//     on the click point
//   - The closest option's box is brighter and slightly larger
// - On release if the line is not connected to anything, create a new box
//   and focus the text input
// - Clicking the center of a box focuses the text input
// - Escape clears the selection
// - Only support text input and backspace. No arrow keys or mouse or anything else.
// - When text input is focused, the box is highlighted

use app::App;
use aspen::runner::run;

fn main() {
    run(App::new())
}
