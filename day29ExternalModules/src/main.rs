// Day 29 of Rust, External module allows code to be compartmentalized,
// which permits organized source code management and provides easy access to where functionalities
// may be located based on the name or purpose of the function. its also encourages
// intuitive coding which can lead to collaboration

// Module xters
// modules can have any name
// they use a hierachical organisation(similar to navigating through folders in a file system)
// They are private by default(can be made public with the "pub" keyword)
// External modules can be a Directory(Can contain additional modules) or can be a file.

fn main() {
    // accessing the flac.rs

    use super::mp3; // takes you up one level from "flac.rs" to "audio", since audio contains mp3(we'd access to its content)
    pub fn sample() {
        mp3::some_fn(); //accesing a function within mp3 file.

        super::mp3::some_fn(); // this can function effectively as line 14 above

        // to access the root, we use "crate" keyword
        crate::codec::audio::mp3::some_fn(); 

        //using "super" twice takes you two step backwards
        super::super::video::h264::some_fn(); 

        // the below synthax works too
        pub fn sample(){
            use crate::transcode as tc; // using "as" keyword to assign alias
            tc::some_fn();
        }
    }
}
