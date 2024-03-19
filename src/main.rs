#[link(name="user32")]
extern "stdcall"{
    fn GetAsyncKeyState(vKey:i32)->i16;
}
fn main() {
    
}

struct Key{
    code:char,
    listen: Box<dyn Fn()->bool>,
    pressed:bool
}
impl Key{
    fn new(c:char)->Key{
        let mut key = Key { 
            code: c,
            listen: Box::new(||{false}),
            pressed: false 
        };
        key.listen = Box::new(move|| unsafe {
            GetAsyncKeyState(c as i32) == -32768
        });
        return key
    }
    fn pressed(&mut self)->bool{
        let v = (self.listen)();
        self.pressed = v.clone();
        return v
    }
    fn refresh(&mut self){
        self.pressed = (self.listen)();
    }
}

#[cfg(test)]
fn test(){
    let mut keys = [Key::new('X'),Key::new('Z'),Key::new('Q'),Key::new('D'),Key::new('S'),Key::new(0x01 as char),Key::new(0x02 as char)];
    let mut status = [false;7];
    loop{
        for (id,k) in keys.iter_mut().enumerate(){
            k.refresh();
            assert_eq!(k.pressed,(k.listen)());
            if status[id] == true && k.pressed == false{
                status[id] = false;
            }
        }
        if keys[5].pressed && status[5] == false{
            println!("Bim Bam Boom !");
            status[5] = true;
        }
        if keys[0].pressed{
            break;
        }
    }
    println!("Fin du programme")
}