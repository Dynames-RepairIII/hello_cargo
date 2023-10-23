pub fn printa_z2() {//公开函数
    let mut n = 3;
    while n!= 0{
        for c in 65..123{
            let a = c as u8;
            print!("{}",a as char);
        }//二级子模块就直接按序打印了
        println!("");
        n-=1;
    }
}
