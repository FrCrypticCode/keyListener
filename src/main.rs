// FFI pour appeler des fonctions externes de Windows => Dans ce cas on appelle une fonction légère d'écoute de touche
#[link(name="user32")]
extern "stdcall"{
    fn GetAsyncKeyState(vKey:i32)->i16;
}

// Construction d'un objet adapté pour s'interfacer et gérer aisément ses touches
struct Key{
    code:char,    // Code touche
    listen: Box<dyn Fn()->bool>,    // Closure unsafe pour récupérer le status
    pressed:bool    // Status
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
// Exemple d'usage pour écouter les entrées générées par un clavier et une souris
// Le GetAsyncKeyState de Windows ne fait aucune distinction entre les périphériques, il regarde les codes touches
fn test(){
    let mut keys = [Key::new('X'),Key::new('Z'),Key::new('Q'),Key::new('D'),Key::new('S'),Key::new(0x01 as char),Key::new(0x02 as char)];
    // Status sert à gérer une pression prolongée sur touche
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
