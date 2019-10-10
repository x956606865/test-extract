// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


/**

TODO:
    * wrap div direct text with p tag
    * handle spa web-page
*/
extern crate reqwest;
extern crate num_traits;

use std::default::Default;
use std::iter::repeat;
use std::string::String;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom, Node};
use html5ever::tendril::TendrilSink;
use std::rc::Rc;


fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}


#[derive(Debug)]
enum NodeTypeEnum{
    Text,
    Element,
    Unknown
}

#[derive(Debug)]
struct NodeInfo {
    tag_num: i16,
    text_length:i32,
    link_tag_num:i16,
    p_tag_num:i16,
    link_tag_text_length:i16,
    td:f32,
    sd:f32,
    score:f64,
    node_type:NodeTypeEnum,
    tag_name:String,
    text:String,
    children:Vec<NodeInfo>,
}

impl NodeInfo {
    fn new() -> NodeInfo {
        return NodeInfo {
            tag_num: 0,
            sd:0 as f32,
            score:0 as f64,
            link_tag_text_length:0,
            tag_name:String::new(),
            p_tag_num:0,
            link_tag_num:0,
            td: 0 as f32,
            text:String::new(),
            children:Vec::new(),
            text_length:0,
            node_type:NodeTypeEnum::Unknown
        };
    }
    fn set_tag_num(&mut self,num:i16){
        self.tag_num=num;
    }
    fn set_tag_name(&mut self,name:String){
        self.tag_name=name;
    }
    fn add_child(&mut self,child:NodeInfo){
        self.children.push(child);
    }
    fn set_text_length(&mut self,len:i32){
        self.text_length=len;
    }
    fn set_link_tag_text_length(&mut self,len:i16){
        self.link_tag_text_length=len;
    }
    fn set_node_type(&mut self,node_type:NodeTypeEnum){
        self.node_type=node_type;
    }
    fn set_node_text_info(&mut self,text:String){
        self.set_text_length(text.chars().count() as i32);
        self.text=text;
    }
    fn set_link_tag_num(&mut self,num:i16){
        self.link_tag_num= num;
    }
    fn set_p_tag_num(&mut self,num:i16){
        self.p_tag_num= num;
    }
    fn set_td(&mut self,td:f32){
        self.td=td;
    }
//    fn set_sd(&mut self,sd:f32){
//        self.sd=sd;
//    }
//    fn set_score(&mut self,score:f64){
//        self.score=score;
//    }
    fn calc_score(&mut self,sd:f64){
        let p_tag_num=if self.tag_name.as_str()=="p"{
            self.p_tag_num-1
        }else{
            self.p_tag_num
        };
        self.score=(((sd as f64)*(self.td as f64)*(((p_tag_num+2) as f64).log10())) as f64).ln()
    }
    fn is_valid(&self)->bool{
        let is_valid_tag=match self.tag_name.as_str() {
            "script"=>false,
            "style"=>false,
            "map"=>false,
            "form"=>false,
            "img"=>false,
            _=>true
        };
        match self.node_type {
            NodeTypeEnum::Text=>{
                if self.text_length==0{
                    return false
                }
            }
            NodeTypeEnum::Element=>{
                if self.tag_num==0&&self.text_length==0{
//                    println!("empty tag:{:?}",self);
                    return false // remove empty tag
                }
            }
            NodeTypeEnum::Unknown=>{
//                println!("unknown tag:{:?}",self);
                return false
            }
        }

        return is_valid_tag
    }
}
fn remove_blank(s:&String)->String{
    let mut ns=String::new();
    for t in s.chars().into_iter(){
        match t {
            '\r'=>{},
            '\n'=>{},
            '\t'=>{},
            ' '=>{},
            c=>ns.push(c)
        }
    }
    return ns
}

// This is not proper HTML serialization, of course.

//fn showTree(indent: usize, handle: &NodeInfo) {
//    let node = handle;
//    // FIXME: don't allocate
//    print!("{}", repeat(" ").take(indent).collect::<String>());
//    match node.node_type {
//
//        NodeTypeEnum::Text => {
//            println!("#text({}): {}", node.text_length,node.text.as_str())
//        }
//
//        NodeTypeEnum::Element => {
////            assert!(name.ns == ns!(html));
////            print!("<{}", name.local);
////            for attr in attrs.borrow().iter() {
////                assert!(attr.name.ns == ns!());
////                print!(" {}=\"{}\"", attr.name.local, attr.value);
////            }
//            println!("{}(score:{},pn:{},td:{},sd:{} ,link: {}, ltl:{}，tn:{},tl:{}):",node.tag_name,node.score,node.p_tag_num,node.td,node.sd,node.link_tag_num,node.link_tag_text_length,node.tag_num,node.text_length);
//        }
//        _=>{}
//
//    }
//    for child in node.children.iter() {
//        showTree(indent + 4, child);
//    }
//}

fn show_text_tree(indent: usize, handle: &NodeInfo) {
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

fn get_max_score(node:&NodeInfo)->f64{
    let mut score:f64=node.score;
    for child in node.children.iter(){
        let child_score=get_max_score(child);
        if child_score > score{
            score=child_score;
        }
    }
    return score
}

//fn get_div_max_score(node:&NodeInfo)->f64{
//    if node.tag_name.as_str()!="div"{
//        return 0_f64;
//    }
//    let mut score:f64=node.score;
//    for child in node.children.iter(){
//        let child_score=get_div_max_score(child);
//        if child_score > score{
//            score=child_score;
//        }
//    }
//    return score
//}

//fn get_direct_child_max_score(node:&NodeInfo)->f64{
//    let mut score:f64=0_f64;
//    for child in node.children.iter(){
//        if child.score > score{
//            score=child.score;
//        }
//    }
//    return score
//}

fn get_global_sd(node:&NodeInfo,sd:&mut Vec<f32>){
//    println!("test: {:?}",node);
    for child in node.children.iter(){
        sd.push(child.td);
        get_global_sd(child,sd);
    }
}

fn calc_score(node:&mut NodeInfo,sd:f64){
    for child in node.children.iter_mut(){
        child.calc_score(sd);
        calc_score(child,sd);
    }
}

//fn print_max_score_node(max_score:f64,node:&NodeInfo){
//    if node.score==max_score{
//        showTree(0,node);
//    }else{
//        for child in node.children.iter(){
//            print_max_score_node(max_score,child);
//        }
//    }
//}

fn print_max_score_text_node(max_score:f64,node:&NodeInfo){
    if node.score==max_score{
        show_text_tree(0,node);
    }else{
        for child in node.children.iter(){
            print_max_score_text_node(max_score,child);
        }
    }
}

//fn print_direct_child_max_score_node(max_score:f64,node:&NodeInfo){
//    for child in node.children.iter(){
////        print_max_score_node(max_score,child);
//        if child.score==max_score{
//            showTree(0,child);
//        }
//    }
//}

//fn show_tag_num(indent: usize, handle: &Handle) -> i16 {
//    let node = handle;
//    // FIXME: don't allocate
//    print!("{}", repeat(" ").take(indent).collect::<String>());
//    match node.data {
//        NodeData::Element {
//            ref name,
//            ..
//        } => {
//            let mut count = 0;
//            for child in node.children.borrow().iter() {
//                count = count + show_tag_num(indent + 4, child);
//            }
//            println!(" {}=\"{}\"", name.local, count);
//            return count + 1;
//        }
//        _ => {
//            return 0;
//        }
//    };
//}

fn get_node_info(handle: &Handle) -> NodeInfo {
    let node = handle;
    let mut info = NodeInfo ::new();
//    println!("{:?}",node.data);
    match node.data {
        NodeData::Element {
            ref name,
            ..
        } => {
//            println!("{:?}",node);
            info.set_tag_name(name.local.to_string());
            info.set_node_type(NodeTypeEnum::Element);
            if info.tag_name=="a".to_string(){
                info.set_link_tag_num(1);
            }
            if info.tag_name=="p".to_string(){
                info.set_p_tag_num(1);
            }
        }
        NodeData::Text {
            ref contents
        }=>{
            info.set_node_type(NodeTypeEnum::Text);
            info.set_node_text_info(remove_blank(&contents.borrow().to_string()));
        }
        _ => {
//            println!("{:?}",node);
        }
    };
//    let child_num=0;
    let mut  child_td_arr=Vec::new();
    for child in node.children.borrow().iter(){
        let child_info=get_node_info(child);
        if child_info.is_valid(){
//            println!("child {:?}",child_info);
            child_td_arr.push(child_info.td);
            match child_info.node_type {
                NodeTypeEnum::Element=>{
                    info.set_tag_num(info.tag_num+child_info.tag_num+1);
                }
                _=>{
                    info.set_tag_num(info.tag_num+child_info.tag_num);
                }
            };

            info.set_text_length(info.text_length+child_info.text_length);
            info.set_link_tag_num((info.link_tag_num + child_info.link_tag_num) as i16);
            info.set_p_tag_num((info.p_tag_num + child_info.p_tag_num) as i16);
            if info.tag_name=="a".to_string(){
                info.set_link_tag_text_length(info.text_length as i16);
            }else{
                info.set_link_tag_text_length(info.link_tag_text_length+child_info.link_tag_text_length);
            }
            let link_tag_num= match info.tag_name.as_str() {
                "a"=>info.link_tag_num-1,
                _=>info.link_tag_num
            };
            let divided=info.tag_num as i32 - link_tag_num as i32;

            let td=if divided == 0 {
                (info.text_length-info.link_tag_text_length as i32)
            } else {
                (info.text_length-info.link_tag_text_length as i32)/divided
            };

            info.set_td(td as f32);
            info.add_child(child_info);
        }
    }
//    for mut c in info.children.iter_mut(){
//        let sd=std_deviation(child_td_arr.as_slice()).unwrap();
////        c.set_sd(sd);
////        c.set_score();
//    }
    return info
}


fn find_body(handle: &Handle, container: &mut Box<Option<&Rc<Node>>>) -> bool {
    match handle.data {
        NodeData::Element {
            ref name,
            ..
        } => {
//            println!("{}", name.local);
            if name.local == "body".to_string() {
//                println!("handle");
                handle_body(handle);
                return true;
            }
        }
        _ => {}
    }
    for child in handle.children.borrow().iter() {
        find_body(child, container);
    }
    return false;
}

fn handle_body(body: &Handle) {
//    show_tag_num(0, body);
    let mut info=get_node_info(body);
//    println!("{:?}",body);
    let mut sd:Vec<f32>=Vec::new();
    get_global_sd(&info, &mut sd);
//    println!("{:?}",sd);
    let sd=std_deviation(sd.as_slice()).unwrap();
    calc_score(&mut info,sd as f64);
//    showTree(0,&info);

    let max_score=get_max_score(&info);
    println!("score: {}",max_score);
    print_max_score_text_node(max_score,&info)

//    let max_score=get_div_max_score(&info);
//    println!("score: {}",max_score);
//    print_max_score_text_node(max_score,&info);

//    let max_direct_child_score=get_direct_child_max_score(&info);
//    println!("max body child score: {}",max_direct_child_score);
//    print_direct_child_max_score_node(max_direct_child_score,&info);
}


// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn main() {
    let resp: String = reqwest::get("https://news.sina.com.cn/gov/xlxw/2019-10-10/doc-iicezzrr1183788.shtml")
        .unwrap()
        .text()
        .unwrap();

//    println!("{:#?}", resp);

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut resp.as_bytes())
        .unwrap();
//    showTree(0, &dom.document);

    let children = dom.document;
    let mut temp: Box<Option<&Rc<Node>>> = Box::new(None);
    find_body(&children, &mut temp);
}



