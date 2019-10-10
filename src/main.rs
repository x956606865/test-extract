/**

TODO:
    * wrap div direct text with p tag
    * handle spa web-page
*/
extern crate reqwest;

mod lib;
use std::string::String;

use lib::node_info::{get_node_info_by_url};
use lib::display::{print_max_score_text_node};

fn main() {
    let url=String::from("https://news.sina.com.cn/gov/xlxw/2019-10-10/doc-iicezzrr1183788.shtml");
    let node_info=get_node_info_by_url(url).unwrap();
    print_max_score_text_node(&node_info);
}



