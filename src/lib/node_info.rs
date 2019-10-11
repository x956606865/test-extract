use super::math::std_deviation;
use super::string::remove_blank;
use html5ever::rcdom::{Handle, NodeData, RcDom, Node};
use html5ever::parse_document;
use std::rc::Rc;
use html5ever::tendril::TendrilSink;

#[derive(Debug)]
pub enum NodeTypeEnum {
    Text,
    Element,
    Unknown,
}

#[derive(Debug)]
pub struct NodeInfo {
    pub tag_num: i16,
    pub text_length: i32,
    pub link_tag_num: i16,
    pub p_tag_num: i16,
    pub text_tag_num: i16,
    pub punctuation_num: i16,
    pub direct_child_text_tag_num: i16,
    pub direct_tag_num: i16,
    pub link_tag_text_length: i16,
    pub td: f32,
    pub sd: f32,
    pub sbd:f64,
    pub score: f64,
    pub node_type: NodeTypeEnum,
    pub tag_name: String,
    pub text: String,
    pub children: Vec<NodeInfo>,
}

impl NodeInfo {
    pub fn new() -> NodeInfo {
        return NodeInfo {
            punctuation_num: 0,
            tag_num: 0,
            sd: 0 as f32,
            sbd:0_f64,
            score: 0 as f64,
            link_tag_text_length: 0,
            tag_name: String::new(),
            p_tag_num: 0,
            text_tag_num: 0,
            direct_child_text_tag_num: 0,
            direct_tag_num: 0,
            link_tag_num: 0,
            td: 0 as f32,
            text: String::new(),
            children: Vec::new(),
            text_length: 0,
            node_type: NodeTypeEnum::Unknown,
        };
    }
    pub fn punctuation_list()->Vec<char>{
        vec!['！', '，', '。', '？', '、', '；', '：', '“', '”', '‘', '’', '《', '》', '%', '（', '）', '.', '?', ':', ';', '"', '!', '%', '(', ')', '\'']
    }
    pub fn set_tag_num(&mut self, num: i16) {
        self.tag_num = num;
    }
    pub fn set_tag_name(&mut self, name: String) {
        self.tag_name = name;
    }
    pub fn add_child(&mut self, child: NodeInfo) {
        self.children.push(child);
    }
    pub fn set_text_length(&mut self, len: i32) {
        self.text_length = len;
    }
    pub fn set_punctuation_num(&mut self, num: i16) {
        self.punctuation_num = num;
    }
    pub fn set_link_tag_text_length(&mut self, len: i16) {
        self.link_tag_text_length = len;
    }
    pub fn set_node_type(&mut self, node_type: NodeTypeEnum) {
        self.node_type = node_type;
    }
    pub fn set_node_text_info(&mut self, text: String) {
        self.set_text_length(text.chars().count() as i32);
        self.set_punctuation_num(get_punctuation_num(&text));
        self.text = text;
    }
    pub fn set_direct_child_text_tag_num(&mut self, num: i16) {
        self.direct_child_text_tag_num = num;
    }
    pub fn set_link_tag_num(&mut self, num: i16) {
        self.link_tag_num = num;
    }
    pub fn set_p_tag_num(&mut self, num: i16) {
        self.p_tag_num = num;
    }
    pub fn set_direct_tag_num(&mut self, num: i16) {
        self.direct_tag_num = num;
    }
    pub fn set_text_tag_num(&mut self, num: i16) {
        self.text_tag_num = num;
    }
    pub fn set_td(&mut self, td: f32) {
        self.td = td;
    }
    pub fn set_sbd(&mut self,sbd:f64){
        self.sbd=sbd;
    }
    pub fn calc_score(&mut self, sd: f64) {
//        let text_tag_num = if self.tag_name.as_str() == "p" {
//            self.p_tag_num - 1
//        } else {
//            self.p_tag_num
//        };
        let text_tag_num = if self.is_text_tag() {
            self.text_tag_num - 1
        } else {
            self.text_tag_num
        };
//        let text_tag_num = self.direct_child_text_tag_num;
        self.score=((sd as f64).ln())*(self.td as f64)*(((text_tag_num + 2) as f64).log10())*(self.sbd.ln());
    }
    pub fn is_text_tag(&self) -> bool {
        let flag = match self.tag_name.as_str() {
            "p" => true,
            "li" => true,
            "code" => true,
            "blockquote" => true,
            _ => false
        };
        flag
    }
    pub fn is_style_tag(&self) -> bool {
        let flag = match self.tag_name.as_str() {
            "strong" => true,
            "font" => true,
//            "li" => true,
            _ => false
        };
        flag
    }
    pub fn is_valid(&self) -> bool {
        let is_valid_tag = match self.tag_name.as_str() {
            "script" => false,
            "style" => false,
            "map" => false,
            "form" => false,
            "img" => false,
            "input" => false,
            _ => true
        };
        match self.node_type {
            NodeTypeEnum::Text => {
                if self.text_length == 0 {
                    return false;
                }
            }
            NodeTypeEnum::Element => {
                if self.tag_num == 0 && self.text_length == 0 {
//                    println!("empty tag:{:?}",self);
                    return false; // remove empty tag
                }
            }
            NodeTypeEnum::Unknown => {
//                println!("unknown tag:{:?}",self);
                return false;
            }
        }

        return is_valid_tag;
    }
}

fn get_punctuation_num(txt: &String) -> i16 {
    let mut count = 0_i16;
    for c in txt.chars() {
        if NodeInfo::punctuation_list().contains(&c) {
            count = count + 1;
        }
    }
    return count;
}

pub fn is_container_tag(node: &NodeInfo)->bool{
    match node.tag_name.as_str() {
        "p"=>false,
        "ul"=>false,
        "li"=>false,
        "code"=>false,
        "pre"=>false,
        _=>true
    }
}

pub fn get_max_score(node: &NodeInfo) -> f64 {
    if is_container_tag(node) != true{
        return 0_f64;
    }
    let mut score: f64 = node.score;
    for child in node.children.iter() {
        let child_score = get_max_score(child);
        if child_score > score {
            score = child_score;
        }
    }
    return score;
}


fn get_tree_sd_vec(node: &NodeInfo, sd: &mut Vec<f32>) {
    for child in node.children.iter() {
        sd.push(child.td);
        get_tree_sd_vec(child, sd);
    }
}

pub fn get_tree_sd(node: &NodeInfo) -> f32 {
    let mut sd_vec = Vec::new();
    get_tree_sd_vec(node, &mut sd_vec);
    std_deviation(&mut sd_vec).unwrap()
}

pub fn calc_score(node: &mut NodeInfo) {
    let sd = get_tree_sd(node) as f64;
    calc_score_child(node, sd);
}

fn calc_score_child(node: &mut NodeInfo, sd: f64) {
    for child in node.children.iter_mut() {
        child.calc_score(sd);
        calc_score_child(child, sd);
    }
}


pub fn get_node_info_from_reqwest_result(handle: &Handle) -> NodeInfo {
    let node = handle;
    let mut info = NodeInfo::new();
    let mut is_patch = false;
    let parent_tag_name = match node.parent.take().unwrap().upgrade().unwrap().data {
        NodeData::Element {
            ref name,
            ..
        } => {
            name.local.to_string()
        }
        _ => {
            String::from("")
        }
    };
    match node.data {
        NodeData::Element {
            ref name,
            ..
        } => {
            info.set_tag_name(name.local.to_string());
            info.set_node_type(NodeTypeEnum::Element);
            if info.tag_name == "a".to_string() {
                info.set_link_tag_num(1);
            }
            if info.tag_name == "p".to_string() {
                info.set_p_tag_num(1);
            }
            if info.is_text_tag() {
                info.set_text_tag_num(1);
            }
        }
        NodeData::Text {
            ref contents
        } => {
            if parent_tag_name == "div" {
                is_patch = true;
                info.set_tag_name(String::from("p"));
                info.set_node_type(NodeTypeEnum::Element);
                info.set_p_tag_num(1);
                info.set_text_tag_num(1);
                info.set_node_text_info(remove_blank(&contents.borrow().to_string()));
            } else {
                info.set_node_type(NodeTypeEnum::Text);
                info.set_node_text_info(remove_blank(&contents.borrow().to_string()));
            }
        }
        _ => {}
    };
    let mut child_td_arr = Vec::new();
    if is_patch == true {
        info.set_tag_num(0);
        info.set_link_tag_num(0);
        info.set_p_tag_num(0);
        info.set_text_tag_num(0);
        info.set_link_tag_text_length(0);
        info.set_direct_tag_num(0);
        info.set_td(info.text_length as f32);
    } else {
        for child in node.children.borrow().iter() {
            let child_info = get_node_info_from_reqwest_result(child);
            if child_info.is_valid() {
                child_td_arr.push(child_info.td);
                match child_info.node_type {
                    NodeTypeEnum::Element => {
                        if child_info.is_style_tag() {
                            info.set_tag_num(info.tag_num + child_info.tag_num);
                        } else {
                            info.set_tag_num(info.tag_num + child_info.tag_num + 1);
                        }
                    }
                    _ => {
                        info.set_tag_num(info.tag_num + child_info.tag_num);
                    }
                };

                info.set_text_length(info.text_length + child_info.text_length);
                info.set_link_tag_num((info.link_tag_num + child_info.link_tag_num) as i16);
                info.set_p_tag_num((info.p_tag_num + child_info.p_tag_num) as i16);
                if child_info.is_text_tag() {
                    info.set_direct_child_text_tag_num(info.direct_child_text_tag_num + 1);
                }
                info.set_text_tag_num((info.text_tag_num + child_info.text_tag_num) as i16);
                info.set_punctuation_num(info.punctuation_num + child_info.punctuation_num);
                info.set_direct_tag_num(info.direct_tag_num + 1);
                if info.tag_name == "a".to_string() {
                    info.set_link_tag_text_length(info.text_length as i16);
                } else {
                    info.set_link_tag_text_length(info.link_tag_text_length + child_info.link_tag_text_length);
                }
                info.add_child(child_info);
            }
        }
        let link_tag_num = match info.tag_name.as_str() {
            "a" => info.link_tag_num - 1,
            _ => info.link_tag_num
        };
        let divided = info.tag_num as i32 - link_tag_num as i32;

        let td = if divided == 0 {
            0_f64
        } else {
            (info.text_length as f64 - info.link_tag_text_length as f64) / divided as f64
        };

        let sbd=((info.text_length as f64 - info.link_tag_text_length as f64) as f64 /(info.punctuation_num as f64 + 1_f64)) as f64;
        info.set_td(td as f32);
        info.set_sbd(sbd);
    }

    return info;
}

pub fn get_node_info_by_url(url: String) -> Result<NodeInfo, &'static str> {
    let resp: String = reqwest::get(url.as_str())
        .unwrap()
        .text()
        .unwrap();

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut resp.as_bytes())
        .unwrap();

    let children = dom.document;
    let mut temp: Box<Option<&Rc<Node>>> = Box::new(None);
    match find_body(&children, &mut temp) {
        Some(node_info) => Ok(node_info),
        None => Err("not found")
    }
}


fn find_body(handle: &Handle, container: &mut Box<Option<&Rc<Node>>>) -> Option<NodeInfo> {
    match handle.data {
        NodeData::Element {
            ref name,
            ..
        } => {
            if name.local == "body".to_string() {
                return Some(convert_body_to_node_info(handle));
            }
        }
        _ => {}
    }
    for child in handle.children.borrow().iter() {
        match find_body(child, container) {
            Some(node_info) => return Some(node_info),
            None => {}
        };
    }
    return None;
}

fn convert_body_to_node_info(body: &Handle) -> NodeInfo {
    let mut info = get_node_info_from_reqwest_result(body);
    calc_score(&mut info);
    info
}
