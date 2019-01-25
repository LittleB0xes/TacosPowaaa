extern crate piston;
extern crate piston_window;

extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;


use piston_window::*;


use rand::Rng;
extern crate find_folder;

const HEIGHT : f64 = 640.0;
const WIDTH : f64 = 400.0;


// Player
struct Player {
	pub x: f64,
	pub y: f64,
	pub dir: char,
	pub pre_dir: char,
	pub speed: f64,
	pub score: u8,
	pub l: f64,
	pub h: f64,
}

impl Player {
	pub fn new() -> Player {
		Player {
			x: WIDTH / 2.0,
			y: 100.0,
			dir: ' ',
			pre_dir: ' ',
			speed: 200.0,
			score: 0,
			l: 60.0,
			h:93.0,
		}
	}

	pub fn update(&mut self, t: f64) {
		match self.dir {
			'R' => {
				self.speed = 200.0;
				if self.x < WIDTH - self.l {
					self.x += self.speed * t;
				}
				self.pre_dir = 'R';
			},
			'L' => {
				self.speed = 200.0;
				if self.x > 0.0 {
					self.x -= self.speed * t;
				}
				self.pre_dir = 'L';
			},
			' ' => {
				let f: f64 = 300.0;
				
				if self.speed - f*t > 0.0 {
					self.speed -= f*t;
				}
				else {
					self.speed = 0.0;
					self.pre_dir = ' ';
				}
				if self.x < WIDTH - self.l && self.pre_dir == 'R' {
					self.x += self.speed * t;
				}
				else if self.x > 0.0 && self.pre_dir == 'L' {
					self.x -= self.speed*t;
				}

			},
			_ => {}
		}
	}
}

// Les nuages
struct Cloud {
	pub x: f64,
	pub y: f64,
	pub speed: f64,
	pub l: f64,
	pub h: f64,
}

impl Cloud {
	pub fn new() -> Cloud {
		Cloud {
			x: rand::thread_rng().gen_range(0,WIDTH as usize) as f64,
			y: rand::thread_rng().gen_range(40,450) as f64,
			speed: rand::thread_rng().gen_range(10,40) as f64,
			l: 60.0,
			h: 32.0,
		}		
	}

	pub fn update(&mut self, t: f64) {
		self.x += self.speed * t;
		if self.x > WIDTH {
			/* retour du nuage au début de l'écran et changement des paramètre de
			   vitesse et de hauteur pour éviter la monotonie*/
			self.x = -60.0;
			self.y = rand::thread_rng().gen_range(40,450) as f64;
			self.speed = rand::thread_rng().gen_range(20,50) as f64;
		}
	}
}



// Les Tacos
struct Tacos {
	pub active: bool,
	pub x: f64,
	pub y: f64,
	pub speed_x: f64,
	pub speed_y: f64,
	pub dir: char,
	pub l: f64,
	pub h: f64,
	pub theta: f64,
	pub w:f64,
}

impl Tacos {

	pub fn new(init_x: f64, init_y: f64, init_dir: char, init_speed: f64) -> Tacos {
		Tacos {
			active: true,
			x: init_x,
			y: init_y,
			speed_y: 200.0,
			speed_x: init_speed,
			dir: init_dir,
			l: 30.0,
			h: 22.0,
			theta: 0.0,
			w: (rand::thread_rng().gen_range(10,15) as f64)*(-1 as f64)
								  .powf(rand::thread_rng()
								  .gen_range(1,3) as f64),  // Sens et vitesse de rotation aléatoire
		}
	}

	pub fn update(&mut self, t: f64) {
		let f : f64 = 200.0;								// Coeff de frottement du tacos dans l'air
		if self.active {
			self.y += self.speed_y*t;
			self.speed_x -= f*t;
			if self.speed_x < 0.0 {
				self.speed_x = 0.0;
			}
			
			match self.dir {
				'L' => {
					self.x -= self.speed_x*t;
				},

				'R' => {
					self.x += self.speed_x*t;
				},
				_ => {}
			}

			if self.y > HEIGHT { //Hauteur max de la fenêtre
				self.active = false;
			}
			self.theta += self.w * t;
		}
	}

	pub fn disapear(&mut self) {
		self.active = false; // Ne pourrait-on pas killer l'objet ?
		self.x = -100.0;
		self.y = -100.0;
	}
}


// Les passants
struct Guy {
	pub active: bool,
	pub x: f64,
	pub y: f64,
	pub speed: f64,		
	pub l:f64,			// Largeur de pixel
	pub h:f64,			// Hauteur du pixel
}

impl Guy {
	pub fn new(init_x: f64, init_y: f64, init_speed: f64) -> Guy {
		Guy {
			active: true,
			x: init_x,
			y: init_y,
			speed: init_speed,
			l: 30.0,
			h: 39.0,
		}
	}

	pub fn update(&mut self, t: f64) {
		if self.active {
			self.x += self.speed*t;

			if (self.x > WIDTH && self.speed > 0.0) || (self.x < -self.l && self.speed < 0.0) { // Largeur maxi de l'écran
				self.active = false;
			}
		}
	}

	pub fn happy(&mut self) {
		self.active = false;
		self.x = -100.0;
		self.y = -100.0;
	}
}



// Le jeu en lui même
struct Game {
	pub screen: u8,
	pub guy1: Guy,
	pub player: Player,
	pub throwned_tacos: Vec<Tacos>,
	pub max_tacos: u8,
	pub tacos_release: u8,
	pub last_guy: f64,
	pub people: Vec<Guy>,
	pub happy_people: Vec<Guy>,
	pub clouds: Vec<Cloud>,
	pub interval: f64,
	pub chronos: f64,
	pub score: u8,
}

impl Game {
	pub fn new(screen: u8, max: u8) -> Game {
		Game {
			screen: screen,  // 0: Accueil, 1: Jeu, 2: Score final
			guy1: Guy::new(0.0,HEIGHT-40.0, 50.0),
			player: Player::new(),
			throwned_tacos: vec![],
			last_guy: 0.0,
			people: vec![],
			happy_people: vec![],
			interval: 3.0,
			max_tacos: max,
			tacos_release: max,
			chronos: 0.0,
			score: 0,
			clouds: vec![],
		}
	}

	fn check_tacos(&mut self) -> bool {
		let mut state: bool = true;
		for i in &self.throwned_tacos {
			if i.active == true {
				state = false;
			}
		}
		state
	}

	fn go_clouds_go (&mut self, many: i32) {
		for _i in 1..many {
					self.clouds.push(Cloud::new());
		}
	}
	
	fn go_people_go (&mut self) {  // Allez les gars !
		let speed = rand::thread_rng().gen_range(30,70) as f64;
		let direction = rand::thread_rng().gen_range(1,3);
		if (self.chronos - self.last_guy) > self.interval {
			if direction == 1 {
				self.people.push(Guy::new(0.0,HEIGHT - 40.0, speed));
			}
			else if direction == 2{
				self.people.push(Guy::new(WIDTH - 40.0, HEIGHT-40.0, -speed));
			}
			self.last_guy = self.chronos;
		}		
	}
	

	pub fn throw(&mut self) {
		if self.tacos_release > 0 {
			self.tacos_release -=1;		
			self.throwned_tacos.push(Tacos::new(self.player.x +self.player.l / 2.0, self.player.y + self.player.h, self.player.dir, self.player.speed));
		}
		else if self.check_tacos() && self.tacos_release == 0 {
			self.screen = 2;
		}
	}

	pub fn check_happiness(&mut self) {
		let hit_tolerance: f64 = 30.0; 				// Distance centre à centre de contact mini 
		for guy in self.people.iter_mut() {
			if guy.active == true {
				for miam in self.throwned_tacos.iter_mut() {
					if miam.active == true {
						if (((miam.x + miam.l) - (guy.x + guy.l)).powf(2.0) + ((miam.y + miam.h)- (guy.y + guy.h)).powf(2.0)).sqrt() < hit_tolerance {
							self.happy_people.push(Guy::new(guy.x, guy.y, guy.speed));
							guy.happy();
							miam.disapear();
							self.score += 1;
						}
					}
				}
			}
		}
	}

	pub fn update(&mut self, args: &UpdateArgs) {
		match self.screen {
			0 => {
				// Mise a jour des nuages
				for i in self.clouds.iter_mut() {
					i.update(args.dt);
				}
			},
			1 => {
				self.chronos += args.dt; // Incrémentation du chronomètre

				self.check_happiness();	
				// Mise a jour des nuages
				for i in self.clouds.iter_mut() {
					i.update(args.dt);
				}
				// Mise a jour des Tacos envoyés
				for i in self.throwned_tacos.iter_mut() {
					i.update(args.dt);
				}
				// Mise à jour des gens qui passe
				for i in self.people.iter_mut() {
					i.update(args.dt);
				}
				// Mise à jour des gens heureux
				for happy_guy in self.happy_people.iter_mut() {
					happy_guy.update(args.dt);
				}
				self.go_people_go();
				self.player.update(args.dt);
			},
			2 => {},
			_ => {}
		}
	}
	

    pub fn input(&mut self, button:&Button, pressed: bool) {
    	match self.screen {
    		0 => {
    			if pressed {
	    			if let Button::Keyboard(key) = *button {
						if key == Key::Space {
							self.screen = 1;
						}
					}
				}   			
    		},
    		1 => {
				if pressed {
		    		if let Button::Keyboard(key) = *button {
		    			match key {
		    				Key::Z => self.player.dir = 'R',
		    				Key::A => self.player.dir = 'L',
		    				Key::Space => self.throw(),
		    				_ => ()
		    			}
		    		}
		    	}
		    	if pressed == false {
		    		if let Button::Keyboard(key) = *button {
		    			match key {
		    				Key::Z => self.player.dir = ' ',
		    				Key::A => self.player.dir = ' ',
		    				_ => ()
		    			}
		    		}
		    	}
    		},
    		2 => {},
    		_ => {}
    	}
    }
}

fn main() {
	
	const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
	const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new(
            "Tacos Powaaa !",
            [WIDTH, HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .fullscreen(false)
        .build()
        .expect("m1");
    let media = find_folder::Search::ParentsThenKids(3,3)
    	.for_folder("media").expect("a1");
    let ref font = media.join("GOODDP.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone(), TextureSettings::new()).expect("m0");
    let tacos_img = Texture::from_path(
        &mut window.factory,
        &media.join("tacos-30px.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("m2");

    let guyl_img = Texture::from_path(
        &mut window.factory,
        &media.join("guyl-sad-30px.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("m3");

    let guyr_img = Texture::from_path(
        &mut window.factory,
        &media.join("guyr-sad-30px.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("m4");

    let happy_guyr_img = Texture::from_path(
        &mut window.factory,
        &media.join("guyr-happy-30px.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("m5");

    let happy_guyl_img = Texture::from_path(
        &mut window.factory,
        &media.join("guyl-happy-30px.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("m6");

    let player_img = Texture::from_path(
        &mut window.factory,
        &media.join("player-60px.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("m7");

    let cloud1_img = Texture::from_path(
        &mut window.factory,
        &media.join("cloud1-60px.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("m8");

    let background_img = Texture::from_path(
        &mut window.factory,
        &media.join("back1.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("m9");

    // Create a new game and run it.
    let mut game = Game::new(0, 20); // screen 0 (start page), max tacos 20
    let mut events = Events::new(EventSettings::new());

    // Creation de l'ensemble des nuages (ici 20)
    game.go_clouds_go(20);

    while let Some(e) = events.next(&mut window) {
        
    	// Lecture des entrées claviers
        if let Some(i) = e.press_args() {
        	game.input(&i, true);
        }

        if let Some(i) = e.release_args() {
        	game.input(&i, false);
        }

        if let Some(_r) = e.render_args() {            
            match game.screen {
            	0 => {
            		window.draw_2d(&e, |c, gl| {
            			clear(WHITE, gl);
            			image(&background_img, c.transform.trans(0.0, 479.0), gl);
            			for i in game.clouds.iter() {
			            	image(&cloud1_img, c.transform.trans(i.x, i.y), gl);
			            }
            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("Tacos Powaaa !"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(45.0, 80.0).scale(1.5,1.5),
		                                    gl).expect("m10");

            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("Be Happy,"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(100.0, 200.0).scale(0.8,0.8),
		                                    gl).expect("m12");

            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("Eat Tacos !"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(150.0, 250.0).scale(0.8,0.8),
		                                    gl).expect("m12");

            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("<a> <z> to move"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(100.0, 300.0).scale(0.8,0.8),
		                                    gl).expect("m13");

            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("<spc> to throw a Tacos"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(100.0, 325.0).scale(0.8,0.8),
		                                    gl).expect("m14");

            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("GFX by Lilo"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(150.0, 550.0).scale(0.5,0.5),
		                                    gl).expect("m15");
            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("Code by Niko"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(150.0, 565.0).scale(0.5,0.5),
		                                    gl).expect("m16");

            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("Eat space to begin"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(100.0, 600.0).scale(0.8,0.8),
		                                    gl).expect("m17");
            		});
            	},
            	1 => {

			     	
			        window.draw_2d(&e, |c, gl| {
			            // Clear the screen.
			            clear(WHITE, gl);
			            text::Text::new_color(BLACK, 30)
		                                .draw(
		                                    &format!("Score: {} / {}", game.score, game.max_tacos - game.tacos_release),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(10.0, 40.0).scale(0.8,0.8),
		                                    gl).expect("m18");

			   			image(&background_img, c.transform.trans(0.0, 479.0), gl);
		                for i in game.clouds.iter() {
			            	image(&cloud1_img, c.transform.trans(i.x, i.y), gl);
			            }



			            for i in game.throwned_tacos.iter() {
			            	image(&tacos_img, c.transform.trans(i.x, i.y).rot_rad(i.theta)
			            					 .trans(-i.l/2.0,-i.h/2.0), gl);
			            }

			            for i in game.people.iter() {
			            	if i.speed > 0.0 {
			            		image(&guyl_img, c.transform.trans(i.x, i.y), gl);
			            	//let transform = c.transform.trans(i.x, i.y);
			            	//rectangle(GREEN, square_guy, transform, gl);
			            	}
			            	else {
			            		image(&guyr_img, c.transform.trans(i.x, i.y), gl);
			            	}
			            }

			            for i in game.happy_people.iter() {
			            	if i.speed > 0.0 {
			            		image(&happy_guyl_img, c.transform.trans(i.x, i.y), gl);
			            	}
			            	else {
			            		image(&happy_guyr_img, c.transform.trans(i.x, i.y), gl);
			            	}
			            }
			            image(&player_img, c.transform.trans(game.player.x, game.player.y), gl);
			        });
			    }
			    2 => {
			    	window.draw_2d(&e, |c, gl| {
            			clear(WHITE, gl);

            			image(&background_img, c.transform.trans(0.0, 479.0), gl);
		                for i in game.clouds.iter() {
			            	image(&cloud1_img, c.transform.trans(i.x, i.y), gl);
			            }

			            for i in game.throwned_tacos.iter() {
			            	image(&tacos_img, c.transform.trans(i.x, i.y).rot_rad(i.theta)
			            					 .trans(-i.l/2.0,-i.h/2.0), gl);
			            }

			            for i in game.people.iter() {
			            	if i.speed > 0.0 {
			            		image(&guyl_img, c.transform.trans(i.x, i.y), gl);
			            	//let transform = c.transform.trans(i.x, i.y);
			            	//rectangle(GREEN, square_guy, transform, gl);
			            	}
			            	else {
			            		image(&guyr_img, c.transform.trans(i.x, i.y), gl);
			            	}

			            }

			            for i in game.happy_people.iter() {

			            	if i.speed > 0.0 {
			            		image(&happy_guyl_img, c.transform.trans(i.x, i.y), gl);
			            	//let transform = c.transform.trans(i.x, i.y);
			            	//rectangle(GREEN, square_guy, transform, gl);
			            	}
			            	else {
			            		image(&happy_guyr_img, c.transform.trans(i.x, i.y), gl);
			            	}
			            }
			            image(&player_img, c.transform.trans(game.player.x, game.player.y), gl);
			        
            			text::Text::new_color(BLACK, 30).draw(
		                                    &format!("Tacos Powaaa !"),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(45.0, 80.0).scale(1.5,1.5),
		                                    gl).expect("m20");

            			text::Text::new_color(BLACK, 30)
		                                .draw(
		                                    &format!("Yout Score: {} / {}", game.score, game.max_tacos),
		                                    &mut glyphs,
		                                    &c.draw_state,
		                                    c.transform.trans(100.0, 320.0).scale(0.8,0.8),
		                                    gl).expect("m21");
            		});
			    },
			    _ => {}
	 	   }
        }
        
        if let Some(u) = e.update_args() {
            // Divers calcul liés au modification du jeu
            game.update(&u); 
        }
    }
}