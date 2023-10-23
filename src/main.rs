mod garden;//引用一级子模块

fn main(){
    garden::printa_z1();//调用一级子模块函数打印
    garden::vegetables::printa_z2();//调用二级子模块打印
}