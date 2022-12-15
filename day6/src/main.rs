use std::collections::HashSet;

const INPUT: &'static str = "srjsssgppnssqzszmzjmjdmmqwqcwqccslsjswjssgbsgsnggqtqnnjznndtndnhddfldfdsfswsjsjjptjpttlwlccpnpgngcncnscncsnnwrwmwggsgdgssvbsstzsstqqrjqqlsqlsqscschhclhccznzcnnzzqppjbpjjqzzbwbqqzdqdjjqtjtbtgtqgtgdgwwsjjbwjjzssrsvsttgqtgglgplgpllcrcncssbrbgrbrllsqlsltthshrrnddwzdzdbbrppdvdqvvpvdvdsslbbmnbnjjrdrnntzzftzftttrsshpspvvrzzqzwzhzszbzjjwqwvqqglgflfwfrfprpddfvdffhzztwtppwwwztzsznzmnnbvbbtqqdhqdqdfffnjnmnlmljmlmnlnddzbbdgdqdnqqtjjtnthttvwwtrwwwgffqcfchfcfwcczhhgjhghpgpwgwffhghzghhtftntnnmlnmlmddqbbfwbfwbwvvfssrggptpltpllbcbzczzlccjgcgvcczjzvvvdtvdvqdqwdqdvdjvddjfdjjqmmfgfsftfzfwzwzfftmfmmqdqccjqcqwwflwlccpssvzzzbnbjnbjjgtgpttbwbrbwrwvrrwccgrgrvvpjvvznvvhddtdldpldppmlplltdllhqhpqhqqbpptgpgjgqjgjffvfdvdttsrsllhzlzrrdrnrjrddqbbjrrvrsstgsttjqjhjbhhnmndmmnrmnnrggmtgmmhshjjfqjjtqtstpstppggtzzrsrwwqffvzvzvqqggqsqtstszttpvtvvvddcncvncvchhnsnpsnnlmnnmqnqjjfwfccmwmbwbswbswwghhzrrptptbbjbtjjgdjgjrgrwgwlglblbnlnbbwnnpbnbmbtmtvvhjjlwlhhszslldwlwdllhjhghqghhqpplhhtjhhnhqhlhjhmmlhlmmrpptvvcczfcccwgcccpgpspdpccfhchffbjbzzwppfbbstbtltpllcssnnctnnqcqhhclhccvlvlffnjjwbwfbfttwvvdvnnnsrstrtmtfthtzzcvzvhzzpmpfpmmmwggdbdqdbbjnbbdqbbrnnnprrwbbcwbbpjjprjjzzvwwvrrthhqvhvnhhzmzszrszsjssclcjjbhjbhjbhjbjhhzbbzttnrnssrbrjbrjjmsmffdlflfzfbzzmtztdthdhldhlddnccgbbmtbbsbzsbzzcsszpzfffprfpfzfrzzhvhffsmmqtjwgjbzhnmrslrmgfjpqcllcgsjdhrshqtlgmqqtfmswfzwtnrswtzdjzclcfmltqgdhcsgvzrdltgfbtqclpppvbqnbqmlhmdbsjbwbdllzpnrwfhmnlgvdwsdjsznnhqzhwntjvcpzdrfwmwwdrttdvzspmbmqggmlmsvwgcjgpvcmplqwfjgpghnfpbctnfhcgngcbdmzhlpcnjpmczzsgfgrdftrzgvmmpdmgcrpcdrjsgczpfjnwpdjpntdngdjwctvcbsjmfwvtsrlhvpswppmfwrwzsbsgbjvljzqjjldmnqnmwsmqnmhmqhhttppbpqlvdcvdbhmbnjzztjrdjdzlvmbdrghtftwdpcwwblsjbgnzwtpztmtmnrpsvzfzncqmrvhcbqcvqvnlngdcllrqlbhjttnmjmhfhdzmjmplcfdqpmwblzsnmpczwcnggcwdvgnjcrrtmpwwqdqpvtbzpdbfnbfcfllntqjlslcsznjvzvsbntrtzwhcbtdmbmwttvhdvdtvrcmprcrrjlgsqddrsmwsrbtbpjrmlbrnsdrjfhjnqjtgjmhzjbjnprvmhtjcdbztwqmrlfflfmcslshtwmwhgvbdslgjhzjlglhllsdphlzngjfwfrlwpnqfhghnqhzhgrszbcwjvlrtmshntszsqplvfbccjwctgtfmqgqjdlgdbwhgvctqtcgfdwvqdwqzddmsbrpftzpqztgzbnplvhftmgpdthnrdhqbltbrmhpcqsfccmqzwmrbnbgbjspslwpjhdqspssqdtnssmjmvzwwfstgzzjrmfczdlznwqpdhbsjqddvffcgfhfdqdrlwcsgcsszdtpqbbsthpwbhdfzmgmcdggfcwcmzfjnfbzbccjhvwhbwfslnqnrwhgrwtlnmrmncnjtbjbdlmqsczppgbcmsdrwlrpjbgrmnhqqfhhsdhmdmpvvpjrnzsvzctmqhpzcvcjfgtlfvqvnvlnprmgrsvrrvtjfndqfsvqdsfbcwlbglmfhfhcfgqdfmclnzhdtppgzzsqgjqncqrbdhlhdjqwjpmbdnfmgdgwbwmnlngnmrhcgqwzvmbjzdvsspjwwdtpnvpftdlqlzfgtscfczsvbrtrqqpgqlvmrtddqplbzsswbgpdzpqfvqpqbndblghmdhmnctdnjbgglmrlvmrmsfgntfdwvqcvvlvbcnwrctvjqhmnsjqccwltbfqpqpmwsfvmnnfqmlmlcchqcdtbvqwcpptvfwrwtbdrlsgnwpmjgnwlprzqjlwqmtmjglbrzlgfbsghwqdmwhrcmfwdmzmflsbngtgndftdpzsqvgqdsfdhplmfcmwpbtvcdmpghmfwqjvhdhfpmbrqpvnbhlftgdtprlztrgnlcldfpjqjqdfrvqtcnzrtjcgzgsslzghlnfhwwjwzdsmpsczclrfmnqjvfmvsqpntsnnnlrfswqtrppzhqgjzlzvrrbhhfhchhvgztpgctcsgssvttszsrdwzwrbmwmspgqhmmfnzqqdbmnbltdmrsvqgddltwczbbjcmplncspgqgmzrndhttsrbvqbpbvhshfqrpqgmmdbhmmtccjcmntmpqhrvhnfnlqqbctsnfzjbphhqwmztgbhqqlbctbsfcszbggzrlcdhwddtjgtqhppzgjsqcddwjsngjrcdflmgwgfnzhjtcwgbqvpwmpgcpdwvqgswwfzcnjgmdpffmqczmsqgpthpmsjlwnrcbzrfshvwftzllwrmfccmlpjnmpjdfpcvjgjpznllmqjwpcflgqgdljtbbjvjlvhhmtvzfnjfnnwrvtlfdbhqphrjghtmlsrplqscsnvjvqdslsbsfzzrjfmchplzgjgdqvzhfphvsjfvnqlgmjfzhrdlmmvfntnzdvrnwqshsmtjnqmwzgpbbzszrsqcvlzjwnmgjhmfqrbvgmfqpswctmvpfcghvdqgstglmzvpfhzfzvhqqdmvrvrttlpwwhqgddzqlrvvdffqtznvlfgjhhmvbmtzjqnnhqzrtbzpqcwpngrdcndcgzwhzgtfwbmwbrpvvczczhwcqwsqzqbqvqftcswtcbzdbpccjhtwbpnwlwwwqwscptlwshrdbmmdcgrmpnnwgjwzszwwdwctfspbfqvdjqtrflshrqlbfgpnrmbszwjpcdzbggphgplcgvwljprzmtncsvfwqchttndhpnzmtdvtjqtcsddtqvcmztmjgwqvjhflrjjtnvfpjrnlvzvwvrpbhrzslmrqqzqhnzqnvtqppmncddphbwwsjczmphsrlltzndtqjgdlqgpnlfcvwntstmrgcrjzmmllpwldnwmwzpvzctmhszspcvtgnqthszzsmtdnzwtfddfctpjhscbqgwfpqmzpvqrzvtbrdjzrqprdgbpmzzfbgqcvcdtsfrffcpqwtvdwvtcqlcsdrzntgrhrspznndslmnvptlphpdqgbblfhmgbpmmfwqzlhvzshhpzgfjldqclngbcbrmmnqqvqmwdnjsglsggqfgjldqfbsqgtrwmpdffqlcwwlfhlpqfgwtssnjwzhgvtwqzmhmgwzwmcggmpmrzqrcsmflqsrbnzvdmjcdbnscstqrqhvddsbjpzwsvzswqhcqmgzlvfcnzjrrffzphmrvdbhqbrwpsfqvfqwhqhcgfvfsfttzcdsrjgjwcgvhllszmplmvgczqsbfldnbvrnqccbprjjdwhmqpdjjrnfdlhzdvlfmrldjlqclbjrrtjfsflphzdcdpfpr";

fn first_packet(input: &str) -> Option<usize> {
    // Loop through a window of length 4
    // Find the first window that contains all unique characters
    // Return the total characters we had to process
    for (i, window) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
        if window.iter().collect::<HashSet<&char>>().len() == 4 {
            return Some(i + 4);
        }
    }
    None
}

fn message_marker(input: &str) -> Option<usize> {
    // Loop through a window of length 14
    // Find the first window that contains all unique characters
    // Return the total characters we had to process
    for (i, window) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
        if window.iter().collect::<HashSet<&char>>().len() == 14 {
            return Some(i + 14);
        }
    }
    None
}

fn main() {
    println!("{}", first_packet(INPUT).unwrap());
    println!("{}", message_marker(INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_packet() {
        assert_eq!(first_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(first_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(first_packet("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
    }

    #[test]
    fn test_message_marker() {
        assert_eq!(message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(message_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
    }
}
