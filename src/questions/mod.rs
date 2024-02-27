pub mod questions {
    pub fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
        let jump = (breaks.len() as f64).sqrt().floor() as usize;
        let mut update_jump = jump;
        //jump to root of n
        while update_jump < breaks.len() {
            if breaks[update_jump] {
                break;
            }

            update_jump += jump;
        }
        //does it break
        //y
            //go back root n
        update_jump -= jump;
        let mut forward = 0;

        while forward < jump && update_jump < breaks.len() {
            if breaks[update_jump] {
                //return index of true
                return Some(update_jump);
            }
            update_jump +=1;
            forward +=1;
        }
        None
    }
}