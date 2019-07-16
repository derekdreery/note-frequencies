use note_frequencies::note_frequencies_32;

note_frequencies_32!(440.0);

#[test]
fn works() {
    assert_eq!(NOTE_FREQUENCIES[69], 440.0f32);
}
