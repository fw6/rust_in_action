use rand::random; // 引入mod包到当前作用域

static mut ERROR: isize = 0; // 全局静态可变量，拥有静态生命周期。初始化为0

struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        // 八分之一的概率返回true
        unsafe {
            ERROR = 1;
        }
    }

    0 // <6>
}

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        // 访问并修改可变静态变量，就必须使用unsafe块
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}
