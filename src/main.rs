use rand::prelude::*;
use theory::*;

fn sign(a: i8) -> i8 {
    if a >= 0 {
        1
    } else {
        -1
    }
}

fn shuffle<T>(val: &mut [T]) {
    let mut rng = rand::thread_rng();
    for i in (1..val.len()).into_iter().rev() {
        let idx = rng.gen_range(0, i);
        val.swap(i, idx)
    }
}

fn counterpoint(notes: &[Pitch], scale: &Scale) -> Option<Vec<Pitch>> {
    // The first note must be a perfect octave, unison, or fifth.
    
    let mut opening_pitches = vec![notes[0] + Interval::Unison, notes[0] + Interval::PerfectFifth, notes[0] + 12];

    // We want only notes in the scale.
    let scale_notes = scale.notes();
    for idx in (0..opening_pitches.len()).into_iter().rev() {
        if !scale_notes.contains(&opening_pitches[idx].0) {
            opening_pitches.remove(idx);
        }
    }

    shuffle(&mut opening_pitches);

    for opening in opening_pitches {
        let res = counterpoint_helper(notes, &vec![opening], scale);
        if res.is_some() {
            return res;
        }
    }
    None
}

fn counterpoint_helper(notes: &[Pitch], so_far: &[Pitch], scale: &Scale) -> Option<Vec<Pitch>> {
    if so_far.len() == notes.len() {
        return Some(Vec::from(so_far))
    }

    let other_note = notes[so_far.len()];

    // If this is the ending, we must choose a unison or octave.
    let mut options = if so_far.len() == notes.len() - 1 {
        vec![other_note + Interval::Unison, other_note + 12]
    } else {
        // Otherwise, we want a consonant interval.
        vec![other_note + Interval::PerfectFifth, other_note + Interval::MinorThird, other_note + Interval::MajorThird, other_note + Interval::MinorSixth, other_note + Interval::MajorSixth, other_note + 12]
    };

    // We only want notes from the scale.
    let scale_notes = scale.notes();
    for idx in (0..options.len()).into_iter().rev() {
        if !scale_notes.contains(&options[idx].0) {
            options.remove(idx);
        }
    }

    // We don't want direct or parallel fifths or octaves.
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx];
        if option - other_note == Interval::PerfectFifth || option - other_note == Interval::Unison {
            let prev_note = so_far[so_far.len() - 1];
            let other_prev_note = notes[so_far.len() - 1];

            let motion = option.semitones_from_middle_c() - prev_note.semitones_from_middle_c();
            let other_motion = other_note.semitones_from_middle_c() - other_prev_note.semitones_from_middle_c();

            if sign(motion) == sign(other_motion) {
                options.remove(idx);
            }
        }
    }

    // Don't exceed a tenth from the other line
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx].semitones_from_middle_c();
        let other = other_note.semitones_from_middle_c();
        if option - other > 15 { // 16 semitones is an octave (12) + a major third (4)
            options.remove(idx);
        }
    }

    // Don't move in parallel sixths or thirds more than three notes at a time.
    for idx in (0..options.len()).into_iter().rev() {
        let interval = options[idx] - other_note;
        let mut count = 1;
        if interval == Interval::MinorThird || interval == Interval::MajorThird {
            for m_idx in (0..so_far.len()).into_iter().rev() {
                let interval = so_far[m_idx] - notes[m_idx];
                if interval != Interval::MinorThird && interval != Interval::MajorThird {
                    break;
                } else {
                    count += 1;
                }
            }
        } else if interval == Interval::MinorSixth || interval == Interval::MajorSixth {
            for m_idx in (0..so_far.len()).into_iter().rev() {
                let interval = so_far[m_idx] - notes[m_idx];
                if interval != Interval::MinorSixth && interval != Interval::MajorSixth {
                    break;
                } else {
                    count += 1;
                }
            }
        }
        if count > 3 {
            options.remove(idx);
        }
    }

    // Don't have both voices skip in the same direction
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx];
        let prev_note = so_far[so_far.len() - 1];

        let is_skip = option.semitones_from_middle_c() - prev_note.semitones_from_middle_c() > 2;

        let other_prev_note = notes[so_far.len() - 1];
        let is_other_skip = other_note.semitones_from_middle_c() - other_prev_note.semitones_from_middle_c() > 2;

        if is_skip && is_other_skip {
            let motion = option.semitones_from_middle_c() - prev_note.semitones_from_middle_c();
            let other_motion = other_note.semitones_from_middle_c() - other_prev_note.semitones_from_middle_c();

            if sign(motion) == sign(other_motion) {
                options.remove(idx);
            }
        }
    }

    // Don't repeat the same note
    for idx in (0..options.len()).into_iter().rev() {
        if options[idx].0 == so_far[so_far.len() - 1].0 {
            options.remove(idx);
        }
    }


    shuffle(&mut options);

    for option in options {
        let mut r = Vec::from(so_far);
        r.push(option);

        let res = counterpoint_helper(notes, &r, scale);
        if res.is_some() {
            return res;
        }
    }
    None
}

fn main() {
    let cantus_firmus = vec![
        Pitch(Note(PitchBase::D, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::F, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::E, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::D, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::G, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::F, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::A, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::G, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::F, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::E, PitchModifier::Natural), 4),
        Pitch(Note(PitchBase::D, PitchModifier::Natural), 4),
    ];
    if let Some(notes) = counterpoint(&cantus_firmus, &Scale(Note(PitchBase::D, PitchModifier::Natural), ScaleType::Dorian)) {
        println!("{:#?}\n{:#?}", cantus_firmus, notes);
    } else {
        println!("Error: No counterpoint :(");
    }
}
