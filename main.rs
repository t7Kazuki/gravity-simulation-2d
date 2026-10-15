use rand::RngExt;

const WORLD_SIZE: usize = 16;
const TICK_SPEED: f64 = 1.0;
const FRAME_DELAY_MS: u64 = 250;
const G: f64 = 0.5;

struct Object {
m: f64,
x: f64,
y: f64,
v_x: f64,
v_y: f64,
a_x: f64,
a_y: f64,
}

fn main() {
let mut t: u32 = 0;
let mut frame = [[0u8; WORLD_SIZE]; WORLD_SIZE];

let mut bodies = [
    Object {
        m: rand::rng().random_range(1.0..10.0),
        x: rand::rng().random_range(0..WORLD_SIZE) as f64,
        y: rand::rng().random_range(0..WORLD_SIZE) as f64,
        v_x: 0.005,
        v_y: 0.0,
        a_x: 0.0,
        a_y: 0.0,
    },
    Object {
        m: rand::rng().random_range(1.0..10.0),
        x: rand::rng().random_range(0..WORLD_SIZE) as f64,
        y: rand::rng().random_range(0..WORLD_SIZE) as f64,
        v_x: -0.005,
        v_y: 0.0,
        a_x: 0.0,
        a_y: 0.0,
    },
];

loop {
    let (dx, dy) = displacement(&bodies[0], &bodies[1]);

    if dx * dx + dy * dy < 1.0 {
        println!("Collision detected at t={t}!");
        break;
    }
    print!("\x1B[2J\x1B[1;1H"); //Clear Terminal

    for i in 0..bodies.len() {
        bodies[i].a_x = 0.0;
        bodies[i].a_y = 0.0;

        for j in 0..bodies.len() {
            if i == j {
                continue;
            }

            let (dx, dy) = displacement(&bodies[i], &bodies[j]);
            let accel = gravity(dx, dy, bodies[j].m);

            bodies[i].a_x += accel[0];
            bodies[i].a_y += accel[1];
        }
    }

    frame = [[0; WORLD_SIZE]; WORLD_SIZE];

    for body in &mut bodies {
        mov(body, &mut frame, TICK_SPEED);
    }

    print_frame(&frame);
    println!("t = {t}");

    t += 1;
    std::thread::sleep(std::time::Duration::from_millis(FRAME_DELAY_MS));
}

}

fn displacement(a: &Object, b: &Object) -> (f64, f64) {
let size = WORLD_SIZE as f64;
let half = size / 2.0;

let dx = (b.x - a.x + half).rem_euclid(size) - half;
let dy = (b.y - a.y + half).rem_euclid(size) - half;

(dx, dy)

}

fn gravity(dx: f64, dy: f64, m: f64) -> [f64; 2] {
    let r_sq = dx * dx + dy * dy;

    if r_sq < 0.25 {
        return [0.0, 0.0];
    }

    let inv_r = 1.0 / r_sq.sqrt();
    let a_mag = G * m / r_sq;

    [
        a_mag * dx * inv_r,
        a_mag * dy * inv_r,
    ]
}

fn mov(
object: &mut Object,
frame: &mut [[u8; WORLD_SIZE]; WORLD_SIZE],
dt: f64,
) {
object.v_x += object.a_x * dt;
object.v_y += object.a_y * dt;

object.x += object.v_x * dt;
object.y += object.v_y * dt;

let size = WORLD_SIZE as f64;
object.x = object.x.rem_euclid(size);
object.y = object.y.rem_euclid(size);

let x = (object.x.floor() as usize).min(WORLD_SIZE - 1);
let y = (object.y.floor() as usize).min(WORLD_SIZE - 1);

frame[y][x] = 1;

}

fn print_frame(frame: &[[u8; WORLD_SIZE]; WORLD_SIZE]) {
for row in frame {
for cell in row {
let symbol = match cell {
0 => '.',
1 => 'O',
_ => '?',
};

        print!("{symbol} ");
    }

    println!();
}
}
