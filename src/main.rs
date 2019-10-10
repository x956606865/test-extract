/**

TODO:
    * wrap div direct text with p tag
    * handle spa web-page
*/
extern crate reqwest;

mod lib;
use std::string::String;

use lib::node_info::{get_node_info_by_url};
use lib::display::{print_max_score_text_node,show_tree};

fn main() {
    let url=String::from("https://news.163.com/19/1009/22/ER33SGIC00018AOQ.html");
    let node_info=get_node_info_by_url(url).unwrap();
    print_max_score_text_node(&node_info);
//    show_tree(0,&node_info);
}



