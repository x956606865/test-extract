use super::node_info::{NodeTypeEnum,NodeInfo,get_max_score};
use std::iter::repeat;

pub fn show_tree(indent: usize, handle: &NodeInfo) {
    let node = handle;
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.node_type {

        NodeTypeEnum::Text => {
            println!("#text({}): {}", node.text_length,node.text.as_str())
        }

        NodeTypeEnum::Element => {
            println!("{}(score:{},pn:{},td:{},sd:{} ,link: {}, ltl:{}，tn:{},tl:{}):",node.tag_name,node.score,node.p_tag_num,node.td,node.sd,node.link_tag_num,node.link_tag_text_length,node.tag_num,node.text_length);
        }
        _=>{}

    }
    for child in node.children.iter() {
        show_tree(indent + 4, child);
    }
}

pub fn show_text_tree(indent: usize, handle: &NodeInfo) {
    let node = handle;
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.node_type {
        NodeTypeEnum::Text => {
            println!("{}",node.text.as_str())
        }
        _=>{}
    }
    for child in node.children.iter() {
        show_text_tree(indent + 4, child);
    }
}

pub fn print_max_score_node(node:&NodeInfo){
    let max_score=get_max_score(node);
    print_max_score_node_child(max_score,node);
}

fn print_max_score_node_child(max_score:f64,node:&NodeInfo){
    if node.score==max_score{
        show_tree(0,node);
    }else{
        for child in node.children.iter(){
            print_max_score_node_child(max_score,child);
        }
    }
}

pub fn print_max_score_text_node(node:&NodeInfo){
    let max_score=get_max_score(node);
    print_max_score_text_node_child(max_score,node);
}

fn print_max_score_text_node_child(max_score:f64,node:&NodeInfo){
    if node.score==max_score{
        show_text_tree(0,node);
    }else{
        for child in node.children.iter(){
            print_max_score_text_node_child(max_score,child);
        }
    }
}

pub fn print_direct_child_max_score_node(node:&NodeInfo){
    let max_score=get_max_score(node);
    print_direct_child_max_score_node_child(max_score,node);
}

fn print_direct_child_max_score_node_child(max_score:f64,node:&NodeInfo){
    for child in node.children.iter(){
        if child.score==max_score{
            show_tree(0,child);
        }
    }
}
