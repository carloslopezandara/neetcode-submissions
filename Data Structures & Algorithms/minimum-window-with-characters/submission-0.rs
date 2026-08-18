impl Solution {
    pub fn min_window(s: String, t: String) -> String {
    // expandir right (hasta conseguir una ventana válida.) -> validar -> contraer left (TODO lo posible.) -> invalido paramos de comprimir y repetimos
    if t.len() > s.len() {
        return "".to_string();
    }

    // convertimos para mejor manejo
    let s_bytes = s.as_bytes();
    let t = t.as_bytes();

    // buscamos los que necesitamos y preparamos los valores necesarios para las windows validas
    let mut left = 0usize;
    let mut best_start= 0usize;
    let mut best_len = usize::MAX;

    // caracteres que necesitamos
    let mut frecuency = HashMap::new();
    
    for c in t {
        *frecuency.entry(c).or_insert(0)+=1;
    }
    // necesitamos por ejemplo solo ABC aunque luego estos puedan repetirse ejemplo AABBC nos da igual de momento
    let needed = frecuency.len();
    let mut formed = 0usize;
    let mut window = HashMap::new();

    //empezamos a recorrer siempre right se mueve solo
    for right in 0..s.len(){
        if frecuency.contains_key(&s_bytes[right]){
            *window.entry(s_bytes[right]).or_insert(0)+=1;

            if window[&s_bytes[right]] == frecuency[&s_bytes[right]] {
                formed+=1;
            }

            while formed == needed {                
                if s[left..right].len() < best_len {
                    best_len = s[left..=right].len();
                    best_start=left;
                }

                if let Some(e) = window.get_mut(&s_bytes[left]) {
                    *e -=1;
                    if *e < frecuency[&s_bytes[left]]{
                        formed -=1;
                    }
                    if *e == 0 {
                        window.remove(&s_bytes[left]);
                    }
                }

                left+=1;
            }
        }
    }
    if best_len == usize::MAX {
        return "".to_string();
    } else {
        return s[best_start..best_start+best_len].to_string();
    }
}
}

