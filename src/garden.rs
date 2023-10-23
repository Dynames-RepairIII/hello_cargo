pub mod vegetables;//公开引用二级子模块

pub fn printa_z1() {//公开函数
    let mut v:Vec<char> = Vec::new();//创建可变数组
    for c in 91..97 {//我的理解是按ascii表逆序打印Z到a中间的值
        let a = c as u8;
        let b = a as char;//默认i32所以需要先转u8再转char
        v.push(b);
    }
    v.reverse();//数组逆序
    let mut n = 3;
    while n != 0{//循环输出，因为没说需要做什么循环我就没用loop的死循环，用的while的次数循环
        for i in &v{//这里直接调用可变数组v会报错，因为编译器担心数组在循环外被改变
            print!("{}",i);
        }
        println!();//用的print挨个输出，因为直接打印数组的调试格式不太好看，但是这里用stdout的刷新就不能换行了，所以单加了一个println作为换行
        n-=1;
    }
}

