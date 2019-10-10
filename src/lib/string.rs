pub fn remove_blank(s:&String)->String{
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
