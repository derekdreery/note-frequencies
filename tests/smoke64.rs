use note_frequencies::note_frequencies_64;

note_frequencies_64!(440.0);

#[test]
fn works() {
    assert_eq!(NOTE_FREQUENCIES[69], 440.0f64);
}
