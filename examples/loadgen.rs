use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::{Duration, Instant};

use rand::{thread_rng, Rng};
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::sleep;

const PARAGRAPHS: [&str; 5] = [
    "Bali is predominantly a Hindu country. Bali is known for its elaborate, traditional dancing. The dancing is inspired by its Hindi beliefs. Most of the dancing portrays tales of good versus evil. To watch the dancing is a breathtaking experience. Lombok has some impressive points of interest – the majestic Gunung Rinjani is an active volcano. It is the second highest peak in Indonesia. Art is a Balinese passion. Batik paintings and carved statues make popular souvenirs. Artists can be seen whittling and painting on the streets, particularly in Ubud. It is easy to appreciate each island as an attractive tourist destination. Majestic scenery; rich culture; white sands and warm, azure waters draw visitors like magnets every year. Snorkelling and diving around the nearby Gili Islands is magnificent. Marine fish, starfish, turtles and coral reef are present in abundance. Bali and Lombok are part of the Indonesian archipelago. Bali has some spectacular temples. The most significant is the Mother Temple, Besakih. The inhabitants of Lombok are mostly Muslim with a Hindu minority. Lombok remains the most understated of the two islands. Lombok has several temples worthy of a visit, though they are less prolific. Bali and Lombok are neighbouring islands.\n",
    "Martin Luther King Jr. led many demonstrations against racism. He delivered his message in a non-violent manner. Some members of his movement later engaged in less peaceful protests. Luther King was detained several times. The longest jail sentence he received was four months. Martin Luther King’s famous 1963 speech, “I Have a Dream”, inspired many African-Americans to envisage a better future. Luther King was an American citizen. Nelson Mandela is a native South African. Their dreams were the same. Their battles were tumultuous. Nelson Mandela was arrested in 1962 for treason. He was incarcerated for twenty-seven years. Nelson Mandela and Martin Luther King Jr. both fought for racial equality. The intolerance of white people towards black co-inhabitants was the catalyst for years of activism. In 1994, Nelson Mandela became the first black president of South Africa. He was the first president elected by the people. Mandela and Luther King have been awarded the Nobel Peace Prize for their dedication to improving civil rights for black people. During Nelson Mandela’s best known speech in 1994, he recited “Our Deepest Fear”, an inspirational poem by Marianne Williamson. Mandela initially avoided violence but ended up resorting to it following the massacre of unarmed black Africans by the government. Martin Luther King Jr. was assassinated in 1968.\n",
    "Several years ago, Channel 4, together with Jo Frost (perhaps better known as Supernanny) conducted an experiment. Forty children, aged six, were invited to a party and divided into two halves. One half was given typical sugary party foods. The other half ate sugar-free foods. The parents were unaware as to which group their child was in. No artificial colourings or flavourings commonly found in sweets were present. Artificial colourings and flavourings have already been linked to hyperactivity. Many parents believe that sugar consumption causes hyperactivity in their children. ‘Sugar highs’ are often blamed for rowdiness or excitability, but is sugar guilty of causing hyperactivity or is it simply a case of ‘normal’ childhood behaviour? As the children ran about and enjoyed the party, the parents were asked whether they believed their own child had been given sugar. The majority believed they had. As the children sat down to watch a magic show, many parents changed their minds. They could not accept that their child was capable of sitting still after consuming sugary foods. The experiment suggested that there was no link between hyperactivity and sugar intake. The children were naturally excited because they were at a party.\n",
    "Travel has undesirable consequences. Pamukkale, in Turkey, is an ancient site consisting of calcite travertines filled with mineral water from hot springs. It is a fascinating and unique phenomenon. It has been under threat of irreparable damage by tourists walking on the basins and by hotels using the mineral water, causing the basins to dry up. Measures were implemented to save Pamukkale. The hotels were demolished. Shoes were prohibited on the basins. The impact of visitors can cause destruction to some of the world’s best treasures. The Great Pyramids have undergone restoration. High levels of humidity created by visitors resulted in a build up of salt, causing cracks. Graffiti carved into the walls was removed. Exploring the world through travel has both positive and negative implications. Travelling can broaden the mind, introducing one to new cultures and experiences. The balance between tourism and the preservation of irreplaceable sites can be precarious. Travelling can offer a first-hand education of historical sites. It can offer the opportunity to appreciate the diversity of nature. Visiting different countries can increase one’s understanding of other people’s lifestyles and perspectives. It nurtures tolerance and open-mindedness.\n",
    "It is known that Sweeney Todd first appeared as a character in a ‘penny dreadful’ in 1846. There is conflicting opinion as to the origins of the story. ‘Penny dreadfuls’ were cheap publications of gore and horror. ‘Penny dreadfuls’ were popular in Victorian London. Although generally accepted as fictitious, it is thought authors scoured newspapers and other stories in search of inspiration. News was mainly spread through word of mouth and subjected to embellishment. Ascertaining the truth is difficult. Sweeney Todd, the infamous barber, has become an intrinsic part of London’s gruesome history. There has been ongoing debate as to whether the tale was based on fact or fiction. Whatever the truth, the tale of Sweeney Todd holds a macabre fascination, nearly two centuries on. Edward Lloyd, publisher of the serial in which Todd first materialised, apparently claimed the character was based on truth. Lack of evidence has left most researchers unconvinced, putting the claims down to clever marketing. Some people still believe in its authenticity. Author Peter Haining published a book claiming Todd was a real, revenge-seeking man. No evidence supports his claims. The Old Bailey holds no record of any such crime. Whether or not the story takes its inspiration from any real event or rumour is uncertain.\n",
];

const CONNECTIONS: usize = 500;
const WRITES_PER_CONNECTION: usize = 10;

#[tokio::main]
async fn main() {
    let mut tasks = vec![];
    for i in 1..=CONNECTIONS {
        tasks.push(tokio::task::spawn(generate_connection(i)));
        sleep(Duration::from_millis(20)).await;
    }

    for task in tasks {
        let _ = task.await;
    }
}

async fn generate_connection(count: usize) {
    println!("Initiating connection #{}", count);
    let start = Instant::now();

    let mut connection = TcpStream::connect(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5555))
        .await
        .unwrap();

    for i in 1..=WRITES_PER_CONNECTION {
        let paragraph = PARAGRAPHS[thread_rng().gen_range(0..5)];
        if let Err(e) = connection.write_all(paragraph.as_bytes()).await {
            if e.kind() == io::ErrorKind::BrokenPipe {
                println!(
                    "Connection #{} hung up unexpectedly after {} writes",
                    count, i
                );

                return;
            }
        }

        let sleep_seconds = thread_rng().gen_range(1..=5);
        sleep(Duration::from_secs(sleep_seconds)).await;
    }

    println!(
        "Connection #{} completed and closed after {}s",
        count,
        start.elapsed().as_secs()
    );
}
