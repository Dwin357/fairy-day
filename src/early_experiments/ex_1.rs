use rust_sc2::prelude::*;


// The original hello-world bot I copy-pasted
#[bot]
#[derive(Default)]
pub struct Ex1;
impl Player for Ex1 {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Zerg)
    }
    fn on_start(&mut self) -> SC2Result<()> {
        //// types in everyone-chat
        self.chat("helloooo!!!");

        // Just trying stuff
        //self.camera(self.enemy_start);

        for worker in &self.units.my.workers {
            worker.attack(Target::Pos(self.enemy_start), false);
        }



        Ok(())
    }

    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {

        // prints in terminal current frame (I think...)
       // println!("iteration:{}", iteration);

       // Well this works
       // println!("camera? x:{} y:{}", self.state.observation.raw.camera.x, self.state.observation.raw.camera.y);
        

        // a way to surrender & stop the program (there may be a better way)
       if iteration > 100 {
           let _ = self.leave();
           panic!("I think we have seen enough");
           
       }

       if iteration == 100 {
           println!("BOOM");
           println!("BOOM");
           println!("BOOM");
       }


        // this is needed for the return type, if nothing else
        Ok(())
    }
}