impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // mientras pueda moverme right avanza, si la ventana falla aumenta left para acortarla
        // eso se gestiona con la condicion especifica en este caso que no exceda longitud s1
        
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut count = HashMap::new();

        for i in 0..s1.len(){
            *count.entry(s1[i]).or_insert(0) +=1;            
        }

        let mut left = 0;

        let mut count2 = HashMap::new();

        for right in 0..s2.len() { 
            //agrego por derecha cuando avanzo
            *count2.entry(s2[right]).or_insert(0) +=1;  

            //mantenemos tamaño de ventana fijo a longitud de s1.
            if (right - left + 1) > s1.len() {
                //quito por izquierda y luego comparo si la ventana actual es la que busco
                let exist = count2.get_mut(&s2[left]).unwrap();
                *exist -=1;
                if *exist == 0 {
                    count2.remove(&s2[left]);
                }
                left += 1;
            }

            if count == count2 { return true }
        }
        false
    }
}
