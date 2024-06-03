const PMAX: usize = 1024;

pub struct Particle 
{
    pub active: bool,
    pub lifetime: f64,
    pub xpos: f64,
    pub ypos: f64,
    pub xvel: f64,
    pub yvel: f64,
    pub tex_id: usize,
}    


const NEW_PARTICLE: Particle = Particle {
    active: false,
    lifetime: 0.0,
    xpos: 0.0,
    ypos: 0.0,
    xvel: 0.0,
    yvel: 0.0,
    tex_id: 0,
};


pub struct ParticleDriver {
    start_search_mark: usize,
    last_particle_mark: usize,

    particles: [Particle; PMAX],
}


impl ParticleDriver {

    pub fn new() -> ParticleDriver {
        ParticleDriver {
            start_search_mark: 0,
            last_particle_mark: 0,
            particles: [NEW_PARTICLE; PMAX],
        }        
    }
    

    pub fn add_particle(&mut self, x: f64, y: f64, xv: f64, yv: f64, lifetime: f64, tex_id: usize) -> bool {

        for i in self.start_search_mark .. PMAX {
            if self.particles[i].active == false {
                // found a free entry

                let particle = &mut self.particles[i];
                
                particle.active = true;               // now allocated
                particle.lifetime = lifetime;
                particle.xpos = x;
                particle.ypos = y;
                particle.xvel = xv;
                particle.yvel = yv;
                particle.tex_id = tex_id;
                
                if i > self.last_particle_mark { self.last_particle_mark = i + 1; }
                if i > self.start_search_mark { self.start_search_mark = i + 1; }

                println!("Activating particle in slot {}, last particle mark is now {}", i, self.last_particle_mark);

                return true;
            }
        }
        
        false
    }
    

    pub fn drive(&mut self, dt: f64)  {

        let mut last_active_particle = -1;
        
        for i in 0 .. self.last_particle_mark {
            if self.particles[i].active {
                last_active_particle = i as i32;
                
                // found an active particle, drive it
                let particle = &mut self.particles[i];

                particle.lifetime -= dt;
                particle.xpos += particle.xvel * dt;
                particle.ypos += particle.yvel * dt;

                if particle.lifetime < 0.0 {
                    particle.active = false;
                }
            }
            else
            {
                // not allocated -> set start mark for the next free slot search
                if i < self.start_search_mark {
                    self.start_search_mark = i;
                } 
            }
        }
        
        self.last_particle_mark = (last_active_particle + 1) as usize;
    }    
    

    pub fn for_each_particle<F>(&self, call: F) where F: FnOnce(&[Particle;PMAX], usize) {

//        println!("for_each_particle() -> {} particles to check", self.last_particle_mark);

        call(&self.particles, self.last_particle_mark);
  }


    pub fn clear(&mut self) {
        for i in 0 .. self.last_particle_mark {
            self.particles[i].active = false;
        }
        self.start_search_mark = 0;
        self.last_particle_mark = 0;
    }


    pub fn has_particles(&self) -> bool {
        return self.last_particle_mark > 0;
    }
}
