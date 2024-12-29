use crate::interpreter::{Interpreter, BLACK_DISPLAY, DISPLAY_HEIGHT, DISPLAY_WIDTH};

fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    {
        use std::io::Write;
        let mut stdout = std::io::stdout();
        stdout.write_all(prompt.as_bytes())?;
        stdout.flush()?;
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input.truncate(input.trim_end().len());

    Ok(input)
}

#[test]
fn user_program() {
    let mut interpreter = Interpreter::new();

    let program_path = get_input("Enter path to a Chip8 program: ").unwrap();
    interpreter.load_program_from_path(program_path).unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn pong_2() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/pong_2.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn hires_stars() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/hires_stars.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn paddles() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/paddles.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn hires_particle_demo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/hires_particle_demo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn puzzle_15_alt() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/15_puzzle_alt.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn ch8_logo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/ch8_logo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn ibm_logo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/ibm_logo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn kaleidoscope() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/kaleidoscope.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn hi_lo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/hi_lo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn guess() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/guess.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn submarine() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/submarine.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn jumping_x_and_o() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/jumping_x_and_o.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn soccer() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/soccer.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn zero_pong() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/zero_pong.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn puzzle() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/puzzle.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn division_test() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/division_test.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn bmp_viewer_hello() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/bmp_viewer_hello.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn blitz() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/blitz.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn most_dangerous_game() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/most_dangerous_game.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn life() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/life.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn hidden() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/hidden.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn brix() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/brix.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn pong_alt() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/pong_alt.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn nim() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/nim.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn reversi() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/reversi.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn blinky() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/blinky.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn tapeworm() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/tapeworm.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn blinky_alt() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/blinky_alt.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn missile() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/missile.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn squash() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/squash.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn breakout() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/breakout.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn breakout_brix() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/breakout_brix.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn tic_tac_toe() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/tic_tac_toe.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn space_intercept() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/space_intercept.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn maze_alt() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/maze_alt.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn space_flight() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/space_flight.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn tron() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/tron.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn vertical_brix() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/vertical_brix.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn animal_race() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/animal_race.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn hires_s_triangle() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/hires_s_triangle.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn tetris() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/tetris.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn sierpinski() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/sierpinski.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn space_invaders() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/space_invaders.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn astro_dodge_hires() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/astro_dodge_hires.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn spooky_spot() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/spooky_spot.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn fishie() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/fishie.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn astro_dodge() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/astro_dodge.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn bowling() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/bowling.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn connect_4() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/connect_4.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn mastermind_4_row() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/mastermind_4_row.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn random_number_test() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/random_number_test.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn sum_fun() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/sum_fun.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn time_bomb() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/time_bomb.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn delay_timer_test() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/delay_timer_test.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn slide() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/slide.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn keypad_test() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/keypad_test.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn framed_mk2() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/framed_mk2.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn trip_8_demo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/trip_8_demo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn rush_hour_alt() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/rush_hour_alt.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn framed_mk1() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/framed_mk1.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn lunar_lander() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/lunar_lander.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn syzygy() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/syzygy.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn tank() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/tank.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn rush_hour() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/rush_hour.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn deflection() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/deflection.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn brick_brix() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/brick_brix.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn wipe_off() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/wipe_off.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn worm_v4() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/worm_v4.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn programmable_space_fighters() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/programmable_space_fighters.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn figures() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/figures.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn addition_problems() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/addition_problems.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn sqrt_test() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/sqrt_test.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn space_invaders_alt() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/space_invaders_alt.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn ch8_picture() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/ch8_picture.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn stars() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/stars.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn min_game() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/min_game.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn pong_1_player() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/pong_1_player.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn coin_flip() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/coin_flip.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn ufo() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/ufo.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn guess_alt() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/guess_alt.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn rocket_launch() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/rocket_launch.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn zero_demo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/zero_demo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn filter() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/filter.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn shooting_stars() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/shooting_stars.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn hires_test() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/hires_test.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn vers() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/vers.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn wall() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/wall.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn sierpinski_alt() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/sierpinski_alt.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn maze() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/maze.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn craps() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/craps.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn rocket_launcher() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/rocket_launcher.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn puzzle_15() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/15_puzzle.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn particle_demo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/particle_demo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn clock() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/clock.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn x_mirror() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/x_mirror.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn merlin() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/merlin.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn pong() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/pong.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn landing() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/landing.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn airplane() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/airplane.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn biorhythm() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/biorhythm.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn cave() {
    let mut interpreter = Interpreter::new();

    interpreter.load_program_from_path("roms/cave.ch8").unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn sequence_shoot() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/sequence_shoot.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn hires_maze() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/hires_maze.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn rocket() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/rocket.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn hires_worm_v4() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/hires_worm_v4.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn trip_8_hires_demo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/trip_8_hires_demo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}

#[test]
fn russian_roulette() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("roms/russian_roulette.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}
