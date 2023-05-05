mod file_reading;
//use std::error::Error;
use std::time;
//use rand::{seq::IteratorRandom, thread_rng};
fn main() {
    use file_reading::file_reading::read_csv;
    //use file_reading::file_reading::shared_words;
    /*let lyrics1 = "Vintage tee, brand new phone
    High heels on cobblestones
    When you are young, they assume you know nothing
    Sequin smile, black lipstick
    Sensual politics
    When you are young, they assume you know nothing
    
    But I knew you
    Dancin' in your Levi's
    Drunk under a streetlight, I
    I knew you
    Hand under my sweatshirt
    Baby, kiss it better, I
    
    And when I felt like I was an old cardigan
    Under someone's bed
    You put me on and said I was your favorite
    
    A friend to all is a friend to none
    Chase two girls, lose the one
    When you are young, they assume you know nothing
    But I knew you
    Playing hide-and-seek and
    Giving me your weekends, I
    I knew you
    Your heartbeat on the High Line
    Once in twenty lifetimes, I
    
    And when I felt like I was an old cardigan
    Under someone's bed
    You put me on and said I was your favorite
    
    To kiss in cars and downtown bars
    Was all we needed
    You drew stars around my scars
    But now I'm bleedin'
    
    'Cause I knew you
    Steppin' on the last train
    Marked me like a bloodstain, I
    I knew you
    Tried to change the ending
    Peter losing Wendy, I
    I knew you
    Leavin' like a father
    Running like water, I
    And when you are young, they assume you know nothing
    But I knew you'd linger like a tattoo kiss
    I knew you'd haunt all of my what-ifs
    The smell of smoke would hang around this long
    'Cause I knew everything when I was young
    I knew I'd curse you for the longest time
    Chasin' shadows in the grocery line
    I knew you'd miss me once the thrill expired
    And you'd be standin' in my front porch light
    And I knew you'd come back to me
    You'd come back to me
    And you'd come back to me
    And you'd come back
    
    And when I felt like I was an old cardigan
    Under someone's bed
    You put me on and said I was your favorite142EmbedShare URLCopyEmbedCopy";

    let lyrics2 = "I can see you standing, honey
    With his arms around your body
    Laughin', but the joke's not funny at all
    And it took you five whole minutes
    To pack us up and leave me with it
    Holdin' all this love out here in the hall
    
    I think I've seen this film before
    And I didn't like the ending
    You're not my homeland anymore
    So what am I defending now?
    You were my town, now I'm in exile, seein' you out
    I think I've seen this film before
    
    Ooh, ooh, ooh
    
    I can see you starin', honey
    Like he's just your understudy
    Like you'd get your knuckles bloody for me
    Second, third, and hundredth chances
    Balancin' on breaking branches
    Those eyes add insult to injury
    I think I've seen this film before
    And I didn't like the ending
    I'm not your problem anymore
    So who am I offending now?
    You were my crown, now I'm in exile, seein' you out
    I think I've seen this film before
    So I'm leaving out the side door
    So step right out, there is no amount
    Of crying I can do for you
    All this time
    We always walked a very thin line
    You didn't even hear me out (You didn't even hear me out)
    You never gave a warning sign (I gave so many signs)
    All this time
    I never learned to read your mind (Never learned to read my mind)
    I couldn't turn things around (You never turned things around)
    'Cause you never gave a warning sign (I gave so many signs)
    So many signs, so many signs
    You didn't even see the signs
    
    I think I've seen this film before
    And I didn't like the ending
    You're not my homeland anymore
    So what am I defending now?
    You were my town, now I'm in exile, seein' you out
    I think I've seen this film before
    So I'm leavin' out the side door
    So step right out, there is no amount
    Of crying I can do for you
    All this time
    We always walked a very thin line
    You didn't even hear me out (Didn't even hear me out)
    You never gave a warning sign (I gave so many signs)
    All this time
    I never learned to read your mind (Never learned to read my mind)
    I couldn't turn things around (You never turned things around)
    'Cause you never gave a warning sign (I gave so many signs)
    You never gave a warning sign (All this time)
    (So many times) I never learned to read your mind
    (So many signs) I couldn't turn things around (I couldn't turn things around)
    'Cause you never gave a warning sign (You never gave a warning sign)
    You never gave a warning sign
    Ah, ah93EmbedShare URLCopyEmbedCopy";
    */
    //shared_words(lyrics1.to_string(), lyrics2.to_string());
    let now = time::Instant::now();
    /*if let Err(e) = read_csv("Songs.csv") {
        eprintln!("{}",e);
    }*/
    let a = read_csv("Songs.csv").unwrap();
    println!("{:?}",a.len());
    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} seconds.", elapsed_time.as_secs());
    println!("test");
    //let p = read_csv("Songs.csv");
    println!("test2");
    /*println!("Hello, world!");
    //use readCSV::level_1::tester;
    //tester();
    use file_reading::file_reading::read_csv;
    let lyrics = read_csv("Songs.csv");
    let mut rng = thread_rng();
    let v = vec![vec![1,2,3,4], vec![2,3,4,5], vec![3,4,5,6], vec![4,5,6,7], vec![5,6,7,8]];
    let sample = v.iter().choose_multiple(&mut rng, 2);

    println!("{:?}", sample);*/
}
