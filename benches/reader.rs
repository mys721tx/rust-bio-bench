use std::io::Read;

use criterion::black_box;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

use bio::io::fasta::Reader;

fn reader<T: Read>(r: T) -> usize {
    let reader = Reader::new(r);
    let s = reader.records().map(|r| r.unwrap().seq().len());
    s.sum()
}

fn criterion_benchmark(c: &mut Criterion) {
    let fasta_100 = "> seq0
CGACTCATCCGCCGAGCTCCTCCTTCCCGTAGTACTCGCTCGAGCAAACACCAACTCATTGTCCTTATCCGACGGTACGAAGTTAAGCTAGGGAATAGCT
> seq1
TGTAGGAGTCTCCTATTACTGTAGTTTTAATGGAACAGATCCCGTCGTACAGCAGCAATAGATGGTTGGGGCCAGATCTTCCCCATGGGCTAGACGTTGT
> seq2
TCGTGTAACTGTTCTAGTATTCAAGATAGATCCGCCAGCCTCCCTTCTCGCTACGGGCGAGACCTGGACGCTGGCTGCCATAACTAATCCCTTGTGGCTC
> seq3
CCGCGAGCATCCTCTGAGATAACACCGCACAGTTGGAACAGCCTGTACTAGACAACATGCAGCAGTTATGTATAGACCAGGTACTTCCCCCGTTGTTCGT
> seq4
AGCACCTTTGTTTCTAAAGCAACTGAAACCTTTTTTCCCCTTACACCGACGAGAAGGCTGCAGTTGTGTATCGAGTACCGTCCAGCAACCTGGATCGTCA
> seq5
AATGGTCTGTGTCGCGGACCTACCATGACAGTGCTGTCCCGCTAATGCATCGCGCTACGTAATGCACTCTGGGGGAGGTTATCATCATGGAGAGTATCCC
> seq6
GGCCTCGTTACGCCACAGGGTAGCATCACTTGGTGTCATCCTGAATGTCTAAGATCATTTCATCTATGACTAGTCGTTCGTGATGGTTCAGAGTCCGTAC
> seq7
CTATAGGCCGGCGGTTAGCCATGGATGGACTCAAAGCGTTGATACAGGGGAAGGGCAGATTCTACTGATAGAGCTACAGGAGGCTTCTCCACATAAAACG
> seq8
TAGGCCATGGTTCCAATACGGACTCGCAGCGCTGCTAGGGAGCATAGGTTTTATACGAGGTTACTCCGACCCGACAATCCGCGCAGGCTGTCGAGAAGCG
> seq9
CCACTCAGGATCGCGAGTACAGGTCGATGGGGAATGGGTTCATGCGACTCACAGTAAATCGTGTAACGCTGGAATGCCGTGTCCACTCGGGGTCTCACTA
> seq10
GTGTGTTAGGGCTGCCTGAACGGTCCCTCCTTATCTTCATTAGGAAAACGTAAGATATGATCTATGTCATCTTTCTTGTATAATGATCGCGGCCATACCG
> seq11
GGCAAGCCCCTGTTGTCTTCAACGAAGTTACGAACCCACATTTCAGACGGCTATCAATCGAGACACTTAACACCGGTATGCCGTCTGGACGGACCATCTA
> seq12
TCCAAAGGAGACCCAGTTCCAATCCCTACATTCACGATCCGGGCGCCTTCGTGCTTGACAGTTACCCGCGGATCCATAGTCGCTTGTCGTAGGATTCAGA
> seq13
TACAAACTATTAATGCTCATTCCATATTTAAAGAGCGTTGTTTCAGTGGCCGTGGGGTTGAGATAGGCCCCAAGATTGTATTCGGTGGTTAGAGCCTGTT
> seq14
GGAATGCATCAACAGAGTGCTGTTGCGGTCTCCGTGTGGGTATAATTACTGTGTACGGTCTATCTGGCCGTCGAAAGGGCCGGGTCCCCTCGCAAGACCC
> seq15
CAACGTACTCATCGAAGTAGAACTACATCGGGAGATGTGGACGTTGACCCATCGTCTGATCCTCGTTCCGTTGCACGAGGTAGCATATCAGGCCCCAACT
> seq16
GTGTAATCGCACTCGACGCCCATCGGGAGGTTGGCGAAATGAGGGTTGTCGTTGGGTCAGCATATGACTCGATAGTGTCAATACCTCCATGGAAGAACCT
> seq17
AGGCATGAGCCAGTCCGTCCATTGGAAATTGTCCCCGATCGCGGTCAACCGCACCTATAGCGTAATAACGACGACAGAGCTTTGAGGGGTCAATTCTGAA
> seq18
TAATATTAAAAAGGGTAAGCGTTAACAGTCTGCTTCCCTCAACACAAGATACCTCCAAAGCCTTGCGTATGAGCATAGAACTTGTCCTTGGGGCTCTCTC
> seq19
GGGTTAACTGATGGATAATACAGATCGCGGGTAGACACCCGAAATTATTTTTCAACCCGTAGTTTCGGTATCAGCGCAGTTATGGTGCACTACGAGATCT
> seq20
CGTAGTGGCTGCTTGCACGGTCCGAGCCGTCGGGGCACCGTCGCGGGCACGATTATCAGTCCACTTGAGTACTACGTCCCACCTGCGCGTTCCAACTACT
> seq21
GCACATCCCAGGCCACGGACGATTGCCCTAGAAGATGCGACTCGTTACTTATTATCCCCCATTTCACATGTGGTACTAGCACGCCCAGGTATTTCAGCGG
> seq22
AAACACATAAATGATGGGACTATGTGGAGCGCTGCGGAGTCGTTGTTACGGCCGCTCGTCTGGTAATTTCCCAGCCACTCGTAGGTAATTCCATAGATAG
> seq23
TAACCATGTAGTTCTAAGGAGTTACTCTACACCGGGGGCGGCCTAGGCCACTGGACGGGGCGAGCCAGTTACCACACCTGTCACAACACGCTACGTGCGG
> seq24
GTGTGGTAACTATGTGGACCGAGGCGATCTGTGCCTCACATTAGGTGAGCTACTGTTGCCTCGCGCGGGACTAGCAATTACGGCGAGGAGGCCGTCGAAA
> seq25
GCAATTTGTAGATTGCTATTAAATCCTCTGATCGGTCTCGTACTCGGAAGAAACCGATTGCCGACCCTAAGGGGCTTAGCTGCCCCAGACCCAATCGCCT
> seq26
AAGTCCGGCTTAGTTTATCAACCCCGATTGGTTAGTAAGGGTTGACCTCTTAGTTCAGCTAAAGGAACTTGCTCAATGTTTTGCCACGGCTGCTAGCCGC
> seq27
CCGGCTCAATTTTTGGTGCACGGTTGCGCGCACTCATTCGTGGCTTATACTGTACGAAGAAAGCGAGACGGGATGATGTTCATCAGTTATTTGACTATGC
> seq28
ACAAATCTCTAATTGAGAATTTCGGACGCAGGTGGTATGTAACTTCGTGTATCTACGCAGCGGTGTGACTCTCCGCTTAACAGCCGACGTAATACGGTCA
> seq29
CGGGTCACATGAAGAGTAGGGAATCGGCGGCAACCAGGCTTGTATCTTGGTGGCGGCCACGTCACTCGAACGTCACGATCTTTTGCAGGGGTCCAGTGAG
> seq30
CTTTGACCGACTTCAGTACCGAACGAGCGATACGGCCATAACCCTATGCATGGGTTGCTCCCGGGCGCCGACATTAACAGTTACGTAATCTCGCACGAGT
> seq31
CATTCTATATAAGATACTGTTACAGGCCAACGAGCTCTACGTGGCGATACCTACTCGATTCCACTGCGCCTTCGGCAGGACTGCGCATTGCTACTGTTAG
> seq32
ATCACGGTGCCTCGACGTTATGGCGGGCGATCATAGCTACTTATCACAGAGGACAATCGAGCCTTGGGATAAGTACCTAATTTCCATAGATGAATCTAGT
> seq33
CTAACACTACAGTGGGACGTGAGTTTGGAGAACGCTAATAATGCGATAATTCAGGTGGAATTGTAAGGCAGTAAGTTTATCGAGGCAGTTTGCGCAGACC
> seq34
GTTGCGCGTGATTAAAGCATGGCGGCGAAGCTTACTGTCCGTTTCCAGTTAACTCGTCCTCTGTTTTCTGCAGGGCTAAAGATGGCTGACGGGTCTTATC
> seq35
TCAATGAATTGTGGGGCGCGTTAGGATTAAATATGCTCCCAAGCATTTCATCGCATAAGTGTCCCTAATCGGAATGCGAGTTTCGCTGGGTAGCTTATGG
> seq36
AGGAGCACCTCCTCTGTTGTCTTAAATCCCACTGGCGTGAGGTGGATCATGCCACGAAGTCGTTTCGCGCGGCCGGCCGGGTTGTAAAAATCTAAGGATA
> seq37
AAACATTGAGCGACTACTTCAGTTTTTCCTGAACTGTAAGTTAACTTCACGTTCTTCCTTTTTCACAGCAGCTCTAAAGAATGCTAGGCAGACAGGAGCT
> seq38
CTTTGGCGGGTTCATAGTTACTCAGGTCTGGTAAAGTGGTCTCTACGCGGAATGTTGTTACCAACACTAACTGCGAATGTCCCTAACCCTAGCTAGTGGC
> seq39
CTAGTAACAAATGCAAGATCGTCCATTCCTCGCCATAGGGGACGTCCCGATCAAGGAAATGACCCCTGAATGTTCGAAGTTGGGAGCTTGAAACGTTCCA
> seq40
AGTACACGTGATGGAAATACTAGCAGTTCAGGAGAGCGGTCAATTAGCACTACCTCGAGTTCTACTTGAATAGCTTCATGCGTAACTTAAGCGAACAAAA
> seq41
GGATCTTAGGCGGCGTAATAGCCTTGGAACTATCCGTTGCTTCCTCTGACAGTCATCATTCTAGGCTCAAAGACATAGACGTTGTCCGACTACGGGGGTG
> seq42
CGAGGCTGGAGATGATCCCAGCCTTTATTTTGTAAGCTCATTATTGAGGTACAATGCCTGACAGGCCTTAGATAGAGTCCGTTTACCGCCCTTTATCAAG
> seq43
CTAATTTTGGAATTGGTGTTGGCTCCTCCCGCCTAAGAATCATGTGCTCGCCGGTCTACAGAACAGTCCCGAATGATGGCGCATTATCACGTATTACGTC
> seq44
GACTGTCAGTCATGCGATTGAGTGTACGAGAGAGCGTTCCTCTTAGTTCGCCAGCGGATCGGTCAGTTCCTGGATTGGATTAGATCCGTCATGTATACCT
> seq45
TGTACAGACCCACTGCTCGTGGAATCTCTTTCCCTGCGCATTGCATGAAGGGGCGCATATTGGCCGTGGAGGCTGGCCCTGATAAACCACAAGTTATATA
> seq46
TACCCTACGGATTAGGTCCGCGCCTAGCATACAGCGCTGGATAAGCAATATATTGCACACGGCAGGCGATGAGCCGATAGTAGGTATATGGGATCACAGC
> seq47
GCCAAGATGACGTAAATCTGTCGCTTGAACGAGAGTTCGCATCTAGACTGCTCCCCGCGCGACTGGCTGGTTCTCCGAACACAAATGTGGGGGGTCGAAT
> seq48
ATCCCACCTATTAGAAAAAGGAGCAGCACCGTCTGTCAGCGGTATATAGTATCCCCTTCTATACGGAACATAGATAGGCTGAAACGACCCTAAAGGCCCC
> seq49
ATTGAGTCGACCGCAACTACGTTCAAAAGGTTGTTAGCGACCCTGCGACTACGAATAGGGCGCGGGGTCTGAAGACGCCTCCCGCGGGTCATCCAGATCG
> seq50
GCCGAGACTGTTCCTGTTAACCGTTAGCGTGAGGTTAGGTATGAACTGTCGTTTTTCATCGTTGCACTCTTGCCAGTTGCCCCCGTCCCGTGAACAAGAA
> seq51
TCTCGCTGGTGGCTTATGTTATAAGTCATATAGGGTATCAGACCTTCTGACAGGTTCCTCAACGCGTTATCAATAGATTGCATATCAACCTTCATACGAC
> seq52
GTCACTGATTTTGGACCTGAAACTCCACATATCCGCAGTCTTTCTATCCGTCCAAAAAATCCCCCATCCGAGATAAGACAATCACACCACTAACAATAGG
> seq53
TTTGCTTGCCGTACTAACACTAAGGGTCGCAATAAGCATGCATATCAACTCAGGGAACTTTCACTAATTTAGAATCGCACTGATTTTGTGACTGCTAGAC
> seq54
TCGACTCACTTGTTCCGCCCGATCAGAAGGGAATGAGAGAGTAGTTATCTGCATCAGCGTACAGTAGCGGAAGCGTCCCTCTTTCCAATGCCTAGCCCGG
> seq55
TTGTGCTGGTGCATCATAAATTGATACGCACATGATTATTTGACTTTTCATGCATCATGAATTAAATACGAACACTTAAACTTATATCGATTCGCTGTCA
> seq56
TCGTTAGCAATGCTGCCTGTTTCTATGTCCACGTTACCGATATGACGGATTGGTTTCTCGGGAGCAGATCGTTAGCGGTCCCCAAACTCAAGACACTTCC
> seq57
CCTCGCACTTTTTCTATGGACCGTCGCTTATGGATCGTTTCGCTTTCAATGAAAATCTAGCCGCCTTAAGGTGATGCACACATCTTGCTTTTGTGCCATT
> seq58
GGTATCGTATTATCGCCTAAGGTCCGTTCCAATCCGTCTACAATCTGTGTGACCTCAGTGACCTAGAGAGAAGGGTAAAGATAAGGCTACGTCAAGGGAC
> seq59
CGCCCACTTCAACGTTGTTCGATCAATGAGGGGCCGCATAAAGGCCAGTGCGCAAGAACAACCCCTCACACTGATCACCGGACATTGATGTAGGGCTTAT
> seq60
GTGGAGGTCTGGGGCGAGCATCATGTCTACTCGAAGTTAACCGATAGGTTACCCTGAACTGAGATTGGTGCCAATCATAACGTATTCGATTCACGATACA
> seq61
GCTAGGCTATATCAATGAGTACAATGTCAAGTATAAAGGAGCACAAGATACAGTAACGTCATCCCCAATAATTTGCTACAGTCGAGGCGTAGATGAACAG
> seq62
CAACTATCCTCCAGGGACTGGATCGACGAACAGGCTCTAAAGTCCAGGAACGGTTCTACAAAGTTCTCCCACCTCAGTGCGATTTAAGACTTAAGACCGC
> seq63
GTAAATTGCAACTGTTCTACTGGTTGTGCCAACGCATGACTCTATATGGCAATCGAGGGGCCGATACTTGACTTCCTTGGACTCCATGACGTTCCCGGTA
> seq64
TAATGTAGCAACTTAGAGGCCATAAGAGGAGACGGGTCATCACCCCTAAGTCTGTTAGATTTTGGTCCTAATCTACGGACGTGGTCGAAAACCTCAATAT
> seq65
AGAAATTTGCACGGATATATAGGGTCCAGCTCCGAGCCGGTTAGAAATGTATCTCTCCTCCTGCCATGACATAATGACATATTGATAAATGCATAAGGTG
> seq66
GAAGGGCGGACAACAGCGGAATCATACGCGCATTACGAGAGAGCCTAGATTAAGCCCAACACAAGTCCAACCTACCGGGCTTGCACGTATTCCTAAAGAG
> seq67
GGCAGAACGGCAACCGACTCGTTGGACCTATTTGTGTCGCCTTAGTTGTGGTGGACCAGCGAGTCAGCGGGAACCGCTGTAGATATTCCAAGGTAGTCTG
> seq68
GAGCGGCGGGGGGTAATGGCATCGTTGGAGCCCCTCGTGAGCCGTATTATCGGGCCGATAACCCAGCTCACGACTTCATCTAATGATTACCAAGTTCCGT
> seq69
CAGCCACAGTCAAAACGAGAGCAGCTGGAGTTTCACGTGCCAATAACCACCAAAAAACCTGCACCCCGGGTTATATGTGGATGCGCTCAACCTCGAAAGA
> seq70
GTTTCCTCCGGTAGTTTGGACCTTTCCGTGTCTCCTCGAACCAGTTATACACAAGCCTTTCCGACTTATTTACGCCTTGTAAGACCGGTAGGCTACTCTA
> seq71
TCTTGGGTGCAAAGTGCCCAATTGATCTGAAGCTCAAGGCACCTTTCCGTCCCGGGAGGCGGCCCAGAAATAATTAAAAATTTAGAACTCGTTACCACGC
> seq72
ACGCTTGATATATTTAGATACCCAGGGTTCGCGACTCAGAGCTCTCTCGCATATTAACCGACGGTACTAAATCAAGGGAATTTTAGCTAGCAGTCACGAT
> seq73
GGAAGTCGCGTCACTTCGGGTCTTTGAGCTGGTTACTGACATGAGAACGCGAGGAAGGAGAGACCCAGCATAGAGACCGAGAGAGTACACAGGCCGGGAT
> seq74
TTCAAAGCGGCTCGATACACGCAGCCTACCGAGGAGCCTACCGTCAAAGGCCCTAAACGCTTTGAGTCCACGTGAAAGGCGCGAGGCTTGGTTAGTGATC
> seq75
GGTCCGGTAGGCTGAGGTATGGACACCAATATGTATTTAAAGATGACATCGCCCAGACTGCGGGCACAAGAGTCCAGCGAGCCAAAATGTGTAGGAGCTG
> seq76
ATTTGACATAAACGCAATGTGGCCTGAAGGTGTGTGGTGAACTCCTTGGTCCAGGGTTTTCTTTAGCAATTGGGGGCCCCGCCCTGAGAAACCGAAGCTC
> seq77
ATGACATGCAAGTGAATCCTGGCAGGAGACAGTTTGTTGGGAACTACTTCGCTGTGCCCAATATACGTTGGAACCGACAATAGGGCTATTGGTTCCAGAC
> seq78
ACGTACAACAAGCCGGTGGCTACCATTCATCGTCTTCCTAGATTTTTGGTTTATAGGTGGCATATCTGGGTTCGATGCGGGGGCACTCGGGCCGAGACTG
> seq79
CATCCGCAACAGTTCGTGCACGTCTCCAGACTGGAGACAGCTTAGATGCTGCCGGCGTGGTGACCTCTCTTCCACGGTTGCTGGAGCGGGTTTATGAACA
> seq80
ATCCATGCACACGCATGCGCTGTTGTGTTAACGCGGTAGCGTAGTCTAACGTAACGATCGACCGGGATCAAGACCGTGGGGGCCACAATAATTCTAGAAG
> seq81
TCTATCCGTATCCGCGAACCCGTTGCGACCTTTTCGCGGGTAGGCCGGAGCGGTTGCCTGGGGTGTGCCCCGAGGCGCGATAGGCGCTCAGGCAACTCCA
> seq82
GGGACTTCTGTCTTCCAGCATGATTATATATTAGCAGTCATTGTTTTGGCACCAAACTCAATTGGACGTAGGGCTTACGTAGGCACACGGCACCATCTTT
> seq83
GATTAGTAATCCCTTCATCGGTCTCATGCGTAGACCTACGGTCGCCGCGGGATTCATACACTGTGGGTTTGCCGCATATTTCAAATGCCCGATTTATAAA
> seq84
CCGCTTTCCGGGACGGATCACGCCCCCATAGTACAGTTTTACGCGGGTGTCGAATACACAAACCCCCTCTTGCATTGACAGCGAGCGACCGTTCTGTAAA
> seq85
TTGCATTAAAGCAGGATGAATGAGCGACTCGTCAGGAGCCCACATCGTCGAATCTCAGAGGTGATCCATGGCGTTACGTAAATGCGTGCGCCAAAGCAAG
> seq86
CAGCCCAACCATTACCAATGATAAGCTTGGTTGATGCGCAGTGTGGACGGCAGGAACTCATGTCCTGTGAACCATGATACTCCACTAAGTTATGTTCCTG
> seq87
GGCTGGGCCATATACTGGGCTAGACAGAGCCTAAGGCGGGCCTTACCTCAGAATACCGAACTGATTATCGTCGACAATACTTCCGGGGTATTAGACGTCT
> seq88
ACGGTGCCCACGAACTAACGGCGGTCCACCAACACTCGGAATCAGAAGAGTTAAAGCGGTCGAACTGGAGCCAGCAACCCCCGAACTTATATAGGGGAAA
> seq89
AATGGTTATTTGTTGCTAGTAATCGCAACATGGTGAGGTGATTAGTCACGTAACTACGGGGCGTTTGGGTCGGGTCGTCGAGCGTCCCGACACAGATTAC
> seq90
TCAGGGTCTACGTTAATGTCCCCCAGCTCACCAGTTGCAAGCCCGCTTAAGTCCATTACCGTTTACCTTCCTGCCTGCATGGTGAGTATCACTCGCTTAC
> seq91
AAGGGAGGGCACCACTAATGTGGCGTAATGCAAAGGTTTAAGGCTTTCTCGTGCGGACTCGCGTCATGGGTTAGATCCTCCGCCAGGTCAGACTATTTTA
> seq92
GTGTTTGGTCGCTAAGTGATGCCCGAATTACTGTCCATATCTGGTAAATGGGACGCTCCATGCGCGAGATATCACGATCTATCACGGTGAGATCTTTACT
> seq93
GCTCGAAGCCCGTGAATCAGATGCGACTGAAATCCGCTCTACGTCGAACGTTCGCCGTGCACGCACGTCACGTGTAAATAGATATTCGAGGAGCGCTCTT
> seq94
ACGGCTAAGAGATTGTCCTAGGTCAACTCGAACCGGCGCGGCCTTGGGGTACATACTTGAGCTAACTAATACACGAGCTTGCACGATACGACTATCTCGA
> seq95
AGGTCAGCCACGATGGTTCGAGGTCTAAGGGTAATTTTCAACTTAGCGCTGGGTGAAGGCTTCTAGTGCGACGGAGTACCTATGTTACTTCAGTTGAAAC
> seq96
TTTGTAGCAAGGCGTTCAAATGATGTAACTTTGTATAACGTTCCGCGCATCGCTGGTCCACAGCACAATCTTTTAGACACCTGAGCAGACACGGATGAGG
> seq97
CTTGGGATTGGACTTCATAACCCATTAACGGGTAGTCTAGGATCAGCGCGCGATACGACGCGGAGAGCTATCGTGGTCCTGATGTTCGCCTGGCTCGCGT
> seq98
TAGCGGACACTGTTAGTTGAAGTATCTTACAGTAAGAGGAAGTGTCTATTATGAGTAATATGGGTCCTAGACAATAACCGCTCAGTATCCGGAACCTTCG
> seq99
GGCAACTAACGCACCTTATAGAATGCCGGTCGCCTCCTGGAGGCGGCAGCGCCGATCTCAGTCATGAGTACAGCGCCAGTTCTTCCACACCGTATCTATA";

    let fasta_200 = "> seq0
AGAAGACGTCATATTTTCCCAGTTTCCGTGATGGTTCAAAATTGAGCAATTGCGGGATCCTAATAGGGCCCACGCCCTCGTGTGAAAGGGGTTGACAATT
> seq1
AACACTTGCAAACAAGGGACTTAATCGTATACTGCATAGTTGCTCATCTTCCGGCCGATCTATAAAGACCATCATTCGCTTCCAGGTCCGTTTAAAACTT
> seq2
CATCGAGCCCCGCACTCTCCGTTACCGCCTGAATGAACGGGCATTGGGGAGGACTCTGCTCTGGTGGGTGTTGAAATTCGAGGCATAATTGAAAGGCTGA
> seq3
TTCACACCGCTTTCTTGTGAAGGTCAACAGGGGGACCCTTATCGCAACCAGCTATGCTTTTCCCCACATTGCGTGACTGCAAATATACGCTTAAATGCCT
> seq4
TAGAAGACGGAACCGACGATCTTCCATGCTAACATCTACATAATGACAAGCATCGGAAGATATTACCACGCTCCGGTACCAGTCCATGAGGGGGGGTTCA
> seq5
CCAGTCCAGCGTTGCCTACAGCAACCAGCTATCGTACCGACTTCAGTGCCGCTATAGCCGCGCACGACGGACGTGTTGAAGCACACACTTGGAACCCGAG
> seq6
AGTGGTTTGGAACTCTGCCGACGTTGTGCGTGAAGCAGCGGAAAACGTATTATATAGCTACGTTTGGTTTATTTGACAGTTTACACTCCTTGGGCAGGTT
> seq7
ACATGCACGCAAAATTACCCATGTAGATTCCTACACACTTTTTGTTGAACGGGACTATCAATCCGGATAATCCTGTCCCTCGCGATAGTAAAGGACTCCC
> seq8
CAACGTGCGGTGACTTGTACATTTGCTCGTATCCATTTAGACCATGAAAGCTGCCGCGTCGCGGTGGACACGATCCTCAAGAATGCTCCCGCTAGCCACG
> seq9
GGGCAACCCAGCGCTGCGACTCGTTTCGATATTGGTCTAATTCCATAGGAGGATATCCTACAGGCCACGCCTATTCCGTCCTCGACTGGTCGTAAATTCA
> seq10
GAAACGAGCAGATACATTAGCAATGGCTAGAGCCTTGTACAAGTTGATGTAGGGTTGGTATCCCTAGCGATGGACGTAAACTCGTGGTAGTTGTTGTGCG
> seq11
CAGAAACTACGGAGGACATTCGCGCGCTCGATTCAGTTGCGGTACCCTACGAACTAAATTGCCGGAAGCAATGGCAAGCATGTACAGTTCTTGACACAAG
> seq12
TGGATATGAGGGAAAAAAGATATACGTGATCAGCCCCGTCCCGTTATGGGGATGATCTACGCTCCTTTTCTCTCGAGGTTACAGCGGACTTATGTGATGA
> seq13
TTCGTCAGCGATTGTGCCGATGAGATTATGGCTTCTGGATGTGTGGTATATGGATAGAAATGGACGCATGTGCGCTGGTCCCGACTGAAACATGTCAGTT
> seq14
ATGATTTAGCGGGTGCGGTAAACTGAATCAGGAACCTGCAATTCCCTGAAGCCCATGCGTGTTGGTTTGAGCACTCCTAACGCCTCAGGGGCAGGTGAAT
> seq15
GTCGGTCAACGGAGCAAGGAGTCGGTTAAGCCATTTCGCTTTTATCCTTTAGTTACGCGCAAACCTTACGATGCACTTGGTTTGTCATTCTCCATGCCCA
> seq16
CCATTCGAACCTGGCGGTGCCAGAAGTAAACTCGGCCACAGAGCTTGATTGTTAAGACGAAGTGCGCTAGTCCCGTATAGATTGATGCCGAGGGGCCTAC
> seq17
AATAATACGTATTTGTCCCACGACCTACCATCGAACCGGATTCATTTCAGGGGGTTCTGGATCGAATGAGCTACTCTCACGTTCCACTAGCCACACTTAC
> seq18
TAGGTGCTTCCTGTGTGAAAGCACCACTACCCCATGGCCTCGGTCGAACAGTTCCCCGGCCGTCAATTTACGGGTCGTGATATTAGACGCGGAATTCGAG
> seq19
CGGAAACAGATGCATAGGTTTGTTCTGAGGTTTACATAGGCCATCGGCGCTAGCAAGGGCGGTACCTATCCTCTTCCTGGGACGTTGTATTTCACGGATC
> seq20
AAATCGCATATCTACTTGGCGCGGCCGGTACATGGGTGCTTCCACTTCGTCACGATATCTATGCCTCGGTTTCAAATCAGCCGGAGCAATAGGGTGGCAT
> seq21
ACCCACTGGCGCACAGAGCAGAACGTTATGAATCAGCACCCTAGGTGTGCGATACGTCCGCGAGTTCATGATGTCACCTTTGACCAAGTCTAATACGCTT
> seq22
TTTATCCGCCCCTGGCATGAGACGACCGTGCGCCCGACCACTTTCGCTTACATACTGCGACGGTGTATCCTAAGCCAGCGCAGGTTTTTATCGACTGATT
> seq23
TTAGTGGCCCCATACAACCGGTCTTCTATCTGGTGTCGGACCCTCGCTAAAATCATGGCAAGCTGGGATTGTGTTTCCGCCGCCTTGCACTGTTGATAAC
> seq24
TTTTAAAATGAGGAATTCCATAGTTATGCAGATCGCGCGGTGTAGATTCCTACCACAAGCTGACACAGTCGCTGTGGCAAGCCCAATCGGCATTCCAAAA
> seq25
GTCCCTTTGGCATTACACTCGTTGCAGCTCCGACACGACTTCGAGACCGCCGCCGTATTATGATCGTGTGGAATGGTATCATAATACGTCTGGCACCACT
> seq26
GACGCCATCTTTAGCCCTCGAGGGGGGCAACCATCGCAGACGCGCATATGTGCGCAGGCTAAGCAATCGACGTGCACGTGCTAGCGCATTGATCCGGTAA
> seq27
TGCGTAGAGGGGTCTTAAGAATAAGCGTCGGTTGCCTGAACGGACAGATGTACAAACACTCGCAAGATCTTTGTAAGATGGCCCAATGTTCCATACTTCT
> seq28
TGGCAATCTGATTTTTTATTTCTTCGACTAATTAATCAAAAGAGTCGCCACCGTGCACTAGGCGGCATTCTGCAATGTTCGAAGATCACAAAGTACAATC
> seq29
CTCGACCCGCATCGTAACAGACGCAAAGTGTGTACGGGAGACCGGCTTTCCTGCGGCTCGTGCGATCACGCCGAGAAGTTAACGCTAGCCTACAGAAAAG
> seq30
CCGCCTCTCGTACATTGATTCAGGGGACTCTGAGTCGTAGTCCCGACAGCTACGGTTTCGACCGTGTCCGATTTAGGGTAGCATCGGATTGGTAGAGTCG
> seq31
GCATCAGTACGCTTGGTATACAATCAAGTAACCCTCACATGCCTGGGCCACATATGTCCGACCTACGAGCACCAATAAAAAACAAGGCTGGATGGCGGGA
> seq32
GTCACGCCGCTCAATCGCATAGGACCATGTAAGAACATCGGCGTCCTCCATTCTTGTAGAGGATGACTGGCGGTATTGATGACCATCTAGGCAAGCAGCC
> seq33
AGAACAGGAGTGCATAGGTTGAGGTAACGTCGCTGGCTATAGAGTCTGCAATGTCTCACGTGCCGGTTAGTCGTTGTCGACCCTGTCAAGTCATCTGGAC
> seq34
ATCAAGCTAGAGGAGTGGAGAAGCACACAATTACAAGTGCTAAGTATGTTCCCGATTTGGATAGGTGTGTGAAGTGGTATAGGCCGGTACTTGGTAGTCA
> seq35
TGTTAGGCAGATGTAATCCCGATAGCCCTTCACCCCTAGGCGTTAGCAGACGTCGGAAGCGCTACTCGACTGCTCTATAATGGTGGCCTCGAGACACAGA
> seq36
AATCGCTTTCCTTCAAGTCGTAAACTCTCAACAGCGAGCAAGTATCGGCCAACGAGTATGAGTTGCGTCGAGAACCACCGGGGCCGGTTTGTGCTAGGCA
> seq37
TAAGTCGTGACTGTCCGTAGCCGTTGGTGCTTAATAGCGTCACATGGTTTCATGTGCCATAACTTATCAGTTTCCATCACTTCAGGCTGACGAAGTATGG
> seq38
GTAAGCTCTTGCTCCCATGTTAATTGAGTCTACTTCTGTTACATTTACCCATTTGGGTTCAAGATTTGTACCGACGTTGTGGGATTAACGCCCATCTGCT
> seq39
AAAACCGTGTCCGGCCCTTCCAAGTACTCTCGAACCCTGAAGTCAACTTGTGCTGATCTTAGGGTAACGAGTATTACACGTGATGCAGAGTCGCGTCGAA
> seq40
TCGCTGGCTAGCCTAGAGCAGGGTATGGGTAAGGCTTTCCTGGGCACATGCACACGCACCAGCTGGCGAGGCCAAACTCGCGGCTCTTAAGGCGCACTTT
> seq41
AATCACCATTCCCCAAGGAATAATGCAATATTGACAGGTTCGTGTACCTCGTCAAAGATGAGTCATTCAACTGAGGGATGTTCATGCAACGGCATTGCGA
> seq42
CGGGAGTTTTGTTGCGACCGGTTTTGGACCGCTGAGCATCTAGGCAGGTGTGTTGATGCGACAAGCTACCTATGCTGCCGCAAAGCAGTTCGGTTAGTCT
> seq43
GTATGTCTCTCCCAGAGTGGCTAATATATAGTAAAACTGCCGAACAGACTGTGAAACCTTTCTGACCCGACCGGTAAGTACGTTCTTGTTCTGCAATTCC
> seq44
ACTCGTCGAACATTCTTAACATTTATCGCAGGGCCCTTCGCTGCCGATAAAGTCTGTAATATCATCGGGAAAGGTCTACGGCAAGTAGATATTTAGATGA
> seq45
CCTTCCTATTAATGCGTTTCCCTGGTGTGAGGGCCTAATACCGAGCCTACTTCCCAGCCGAAATAATCTCAGAAGTTCTATTTTGGGCGGTTTTATTCAA
> seq46
TACGCGGTAGGGGATTAGTATTAGCCCCGCAGGGGGATCGCGATATGGCTCCTGTGACGTTGAGACCATCGGACGAGTAGCTAACTCGCGCATTGCTTGT
> seq47
ATAGGGACAGCGACGAGGGGTAAGAAATTGGTGCCCAAAAGGGACCAGCCCGTGGACTATACCTCTAATGCACTTAAGACACGGTCGAGCATTTCCAAAG
> seq48
GACTTGAGTGCCACACGTGAAGTTTAGCCGTCTAGGATTCGTTTCTTCCCCGCCGCGCTACACTAAACTCTCAATGCCCCGAGTTAGAGTTGCTTACAAA
> seq49
TTCTAAGCAATTTAAGCATGTGTCTCACCGCCAACTTCTGCACATGAGGGAGGGTCCCCAGACGCAACCTAATGAAAATATCCCTGTCGGGCAACACGCG
> seq50
GCGTTCGCCTCGGTTAACGTGCGACTACAGACTCTTTCTCTGTTAGAGCGACTACTGCGGCCCGCCCGAATGTGGAAACCGACTGCTCTTAATAGCTGGC
> seq51
GCGAATTCAGACCTCGAGGTGTTACGTAGTCGCTCGCAACCCGTACGGGCGAACCATGGACCGTTCTGAAATCAGAGGTTTATTCTTGCGTATGTGTTTA
> seq52
CGAACTGCGGCGTCCTGCATGGGAATTACCGGATCCGACGTGGCCTCATGGCTGGGATCTCTACGAGGGTAGTGAGCGGTTCTTGGGTTAAGTCTGTCTT
> seq53
CTTATACGATGAACATGCAATCGTCTTCACCATATTGTCGCTCCTGCCTGTCGAACAGCATCCTATTTAAGCAGGTGACTGAGGAACAATATTATTCTTT
> seq54
TGTACGAGTGCTGGACGGAATATGCCTCCCGCAAAATTGTCTGCGAGTGTATTCGGACCTATTAAGTACGATCTCGATGAGGCGATCACGGCCGGCTCCA
> seq55
TTCCCAGGAGTGTGCATATATACGTAGAGTACTCAACCTTACACAGTTTCTCATAAGCCTCATCCAAACTGTAACTTGCTCTTACTGCGTAAAGGGAAAT
> seq56
CGCGTGTCGGAGACCAATATGTCGAGGAATAACGATTTTCGAGGAACTTTATCAATGCGCATGCATACAGGTGAAATTCCCTCGCGCGAATGTGAGGTTT
> seq57
GGATTTCATACTGCTTCGGATGGACTTTTTCGCTAGTCGCGACACCTTGGGGTTTATTCTGTGATGTCGGCAGGCTGCAGAAGGGACTCAAACCTGGGCC
> seq58
TATCACGAACTTAGTATAACTCTCAGCTATCCGGTGCGAAGAATCTTCCTGGCGGGCTGTTATACAGAATACAGCTCGCCCAATTATTCCCACGGTTCAT
> seq59
CTGCGACCTGCGTAGCATTTGGGTTCCAAAGCCCAGGCCGGCGTTTTGGTAGGTGACGCGCTATGGTGTCGTAGTTTGGCGCTACTTATTCATACTGTGC
> seq60
AACAGAGTGGACAAGCCGACTACTTAAGTAGTACGGTGGCACCACTTACATAAGGGAAATCGACCATTTGAATAGGCGGGCACAGACAATCCAAATGCTG
> seq61
CGAAGAGATGGTCTTGTCAACTGCTTGAGTAATAACTAATGCACCATATAACGTGAATCCCAGTTCGGAGGGCTACTTTGTGGCCCTCGCATATATGAGA
> seq62
AGCTGCGAAGACCTTCTTTACGCGCCGCTGGAGACGCGTTCCATAACAAACACCTGTTGGACGTTATGCCTTACTCAGTAACGGGGTGCGCCCACCGGGC
> seq63
CTGCGTGCAGAGTAGAGTTGAGATCTTAGCGCACTTAAAGCGTTGAGGCGCCCATGGGGAGTTCCAACACTTTCCTTTATGGCCATTTCGAGTCGTAAAT
> seq64
CCCCGGATAGGAATTCCTTCCATAGGGTACAGTATCGCAGTGAACGCGCTTCAAGCCTAACACGGAATTTGTCCCCTATACGCATCCGATGCGATAGTCC
> seq65
TAAACAAGCCGTACGTCCTATGAGGTGCGCGACACATCCGTTCGGCCAACATTGCTTAGCCCAGACGGGGTACTCCTGACTCAAGTTGTTATGCGAATGG
> seq66
ATCAGCAAACTGTCGGATGTACGGAAATTTACATTGTCGGATGAGTAGCGGTTCCGCCACCGGTCGGGCCCAAGGCCGGAGCCAACCTCGGCACACATCC
> seq67
TTACAGAGCGATTATCCCCTGTTATCAAGGACTGGGACAGGCTCACGGCGTACGCTCTCTTTCTGAGCGTCATAAGGTCCAGCTATAGGTCAGCACATAG
> seq68
TAGTTGTACAACCCGCTTAGACCGTAGTGGTATTGTACATGGACACGGCTCCTCTCCACTGCTGAAAGCAATAAGTTAAGGCCTAATAAAACATACCTCC
> seq69
CATAGGGGGAAACCCAAGCAACTATGCAGTATTACACCCTACAGCGCAACGCCGAGACGGACGACATACTAAGTTGGAATACGGCGCGTTCCTCCGTCCC
> seq70
ATAGTGTACTCGTCAGCTAGCTCTCAGATTTTGAGGTAGCGCATGAGGATTCCGACAATTTGACAGCGCAACACGTACCTGGAGTTCAATACGCTGCACA
> seq71
GAAGGGTGAGAGACTGATATGGGCTCGGCGGGACTATGGTTTATCTATAATGGTGTAGTAACTCGTGTATAACCGGAACGTATAACGGAAGCGGATCTAG
> seq72
GTTAGATCGTCGCCTTCCCAGATCGCCTTAAATTTGGGGTTGTCGGGCCAGTATGCGTAGTCGTGCACTGCTATCATCAGCCCGGGGCGAAGGTAATAAC
> seq73
CGCATTACTAAAACGTGCGCCCGCTTTGTGTGCGTACGTGTAAACGATGGGTCCCTACATTCAACCCAGAGATCTCAGTAACTCTATTCCCATCGGGTGC
> seq74
GAGGTCGGAAACGACTAATTTCTCTAGGTTTCGAAATCGTGGGTTGGCGTAAAATGTGAGAGCATACGTCCGCGAACACCGGCTTGAGGGTGGCTGCCTC
> seq75
GTTAAAAGGTAGTACGTATTACAGTACCCAGTGTGCTTGCGCATATCCCATCCAGCATGCACCTGGCTGGTCAGGTTGATCCGCTCCCACTCCAAACTAG
> seq76
ATCGGGCTACGAAGCAAACTAAGTCACCGGGCAACTTGCCCTGGTAGAAACGTGTCCACCTCACAGCTAGAGTGGAACCGGACAATCGAGAGGCAGTCCC
> seq77
CTGGACAGTAAGTTACAAAGGTTGACTAGCTACGCGTATACGTTTTGCCTCTACGTGTGAAGTTCGTCAAGACTAAGCAAGATGCCCAGGCAAGTCTGAT
> seq78
GTTCATGGCCCTGGGGAGGGTCATCGTTTAATGTGTGCTTGGGAGGGCGACCCTCGTAAAATTTGATCCATCAATGCACTCTAGACTCCACGACAATACT
> seq79
GGGGACCGCGTCTTTCCAAAGCTCGCGATTCGACGAACAGTTGGCGAGGACATCTCTTAATAACGTTAGCGAAGTGTTCTGCAGCGGAAAGTCCAACTTT
> seq80
GGAGACACGGCTACGGCAAACTGATTCCGTACGCGCACATATCTTATGCCTCTATGTGAGTCTAAAATTGCGACAAAAATGCTTGCAGGAGGTTGGAGAA
> seq81
TGTCAAAATTCCGGACGACCACTCGGGTTGACTACCACATCGTCGGCTTTCCTCTAACTAACTTCAGTGACAGGGCAGTCCGTCCGCGCTTATTCGGATA
> seq82
TCTTGTCTTAATTCGCGTGCAAAAAGTAAGGCTCCTTTCTGAAACCGGGAACTCGAGGGGTATGGGGATTCGGGTTAGCACCTCGGCTTAGATCGTAGGC
> seq83
CCGAGTTACTTTGCCTGGTTGCTTATCTTTTAATTTTACCCTTATGGTTATGCGTAGCGCTTGCTAAGGTGGAGTATCCCAATGGCGTTCCATACGTGGG
> seq84
GTCCTCAACTAGATAGTGCCAGAGAGTGTGAGATGTCCAACACCCAACGCGTGTCATGGAAGTACCCCCTTCGTTTCGCATTCTTGTGACCCCTACGACA
> seq85
TAATCTAATTGACACCGTTTTCGTGACGAGCATGTGCGCGGTATGACATACCACGTGACCTGTTCCGCGCCATCACCGTGCCGCTGGACAATGCGTCAGA
> seq86
GTACGGGTGTGGTGCGTCGGACACGCCAACTTACGGTCTATTCCATGAATTGCGGATGTCCCATAGCCTGCTGGGCTAAGATTCTCTGTCGGCCCCAGGG
> seq87
CGTGGTGTCGGAGCGACGATGAGCTCTGCGCGTACGACCGGAGGTCGCGTATAAACTTAGGCTCCAGTACACTGCGAACGAACGCGTGTTCTGGCTACAT
> seq88
TGAGATGCCTCGTTCGTCTTCACCTGCAGGCGGTATTCATCGGTACTGTGGCCTCGATTGGTCGTAGGTGTGCGGGGTATCTCATCTGCGAGGCGTGCGA
> seq89
TTTAGATTCTTTCCAGGAGGCAAAGGATACGAACCTGCCCATACTTATTCCTCGACCTGCGAGGTGACTCGGTCAACGTTGCCCAAAGCGCTAATGTAAC
> seq90
CGCTTGCAGATTTGCCACAGCCTGGGCTATACCTTTTGATGAATTTTAGTTCACTCATGTCCCACAAGCTAATCTTTATTAATATCGGCGGAAAGGTAGA
> seq91
GGAAGGGTCGCGCGGACGGTCGTAGGGGGCGCTTTGGTTATCCCCATGCCCAGATATGGGTCTGACGTCTATGGCCGCCGCGACCGTTGGGGTAAGGCGC
> seq92
TATAGAGCACCCCAACGTGAGTTTTCCGCTCCTTGCTTAACCATGTGTACAGTCTCCGTTAGTATATCCAGTGCAGATGGCAATACGGTACCATTTTTTG
> seq93
TAATGCAGTAACATCGGGACATCTCCGTCGCCCTGCCGGTGTGACATAACATCAGACTCAGCTACAAGTAACAGGCCCCATGTTATGCCGTCGAAGGGAC
> seq94
CGGTCGAAGAAAAGTTGAGTCCTGTGCGGTCCGACCACGGCCGCGGGCTCGCATTCAGACGGTCGAAGCGCGATGGGCGTATCAGCCCAAGGGACGTTTT
> seq95
TCGTAGACCAATGGATCCAGTACGTCTAGTTTCTATTTCGCTTCCCGTGCATTATGGGTCTGTGTTAGCGTAGAGCAAACAGGTACCTACTGCACAGCTA
> seq96
CCTACGTCGCAATGTTTGGCGTCCTTCGCCAGGCCTGTCGTTTAAGGAACCATCTGGTCTTTCTGGAGAAGGAGTACGACTCTAAGATTGTATTTTGACT
> seq97
AGTGCCGGACGCGAACTTTGTTCTCCCGTGACCAAAGCAACCGACGTAACTGAGAGAGACCTTGCATAACATGGTTTCAATGGCCCTGTTTACTCACCAC
> seq98
CTAGAAGCGATTCGGCAACGTTGCGACCCAAATGCGACCTTCCGACGAGAAGAATGTTTTGACCCGAGCCGAGAGCTACTGTAACCGCATCTTATGGTAG
> seq99
GAAGTAGTAACACGAAGGAGGGTTAAAGGTTGACCTCCTCTGCGATCTAGACCCTACTTCAACGCCAGAATCCACCACCGTACAATCGGTACCTGAATTC
> seq100
TGGTGGAGGCGGTAGTGGGTTGCTCGGATCAGCGTGTTCGTATTAACTTCCCCGTCGGACCCTGCTGATGCGTAGTGGCTTACCGATCAAGCCGTCCCTA
> seq101
GGGGTCTCGAGGGTGTTCTAGTGTGATGATGTGTGCTGTGTCGTGACTCGGTGGCCGGGTTTGCGTTGCGGATTCCCTATTACATACCCCAGATGGAGGC
> seq102
GGAGGCCGTTTGGTTCCACTCTTTGGATATGGGGAGAACGGGCACTATCCGAGCCTCCTCGGTCGTTTATAGGGACTCATCAATAGGACGACCGTGCTAT
> seq103
GTCGTTAAGTATTCCTGTGATGTCCAGCAAAAGGAACGAAGAGCTGGCCCGGGAGGGAGTATAAGTCAACCGTTTCTCTTTCGCTTCGCGCGCAATCAGC
> seq104
TTCACAACAACAGCCCACACCGATTCTCGACAAGGCGAGCCGTTATTAACTACTGACCAGCAGAGTTGTGGGGTTTGATACACCGTGGGCCCTATCAGTG
> seq105
CTCCGGATGGTAAGGGTAATTGGGTCTGATAGGGCGAGAAGGATTGCCAATGTGCCGCGGATGTTTCACTAATAAGACTCCGCACATTGGCAAAACTCTA
> seq106
AGGGTTCGTAACTGTCCGTTCCGATGGGTCGTGAATTACGCATACAATTTTCAAGTTTAATACATTTAGCGAGAAATATGGACGGTCTGGTTTGACTTGA
> seq107
GATCCGTCTTGCTCGGCAAAACCTTTATCGCAAGTGCCGTCACCCCATAACTTGCAGGCTGTGCGCAAAGCTACGGGAAGGAAAGTTACTCTTCACAGCG
> seq108
CTATGGGAGAATCACCCGTCCAATATGGTAGAGATAGCTGGGATGTTCCCTCCACAAGGTATTGGCAGGGGTTTAGCCGTGGGTCTGACTTGTAGAGTTG
> seq109
CTGTCCGCCAAAATACGTCATGCCACTGTCAGCTGTGAACTCTTATCCAATGTGGCGGGCGCACCGTTGCTCCGTTTTCGTAGATAAGGGCCACATTTCT
> seq110
CAGGCGTCCCCTCCTTTTGAGGGCTTCAACCGGAACTTTTCGCTGACGTTGCACCGCTATGAGGCATATCAGAGTGCGGTCACATCTTTACATTCCGCGA
> seq111
TGCTTCGGCATGTGGTCGTTAGCATGGTGACCGTACTTCGGATTCTTCTGGCGGCAACACATTGACGCTACAATAATCGCCAACCAATAGGTCCATCAGC
> seq112
ATACCACGACCGAAAGAAGTATGTGGACAGCGTACACGGGGGTGTACAAGGATGTTTAACCTCTCCCAGCTGGGCACAGAGACCAACTAAATAAGACCTA
> seq113
TGCCCCCCAGTGCGGAAACATACCAGTTCACCACTATCCGGGATGAGAGCTATGGGACGGACATACGTACCTCCGCATGGGTAGTGTGAGTGGTACCAGG
> seq114
TTTTGGTCTTATCCCTCCTTGTGTACGACCGTCACGAGAGTCCTAGTTAGGATGTCGAAATACGTAAACCATTAAGTGTTGCGGGTGTCGTCGCCCTTCA
> seq115
GCTGAATGCGGCATTTCACGACTCGGGGAGTGTTTCTATCACAGATGTGACTTAGATTTAGGGCCATACCCACCTATTCAAGGGATAAAAATACGCATTG
> seq116
GACCTCCATGTACGACGAGACGGATGTCGCTGCACGGAGCTGTCGCCATTTGAGGGTGCAAGACCGGTGCGTTACATCCCGGAACTCTGATTGAACAATC
> seq117
TTCCCTCATTTGGACGGGTGCGGGTCGCCTGTACCATATGCCTTGCCACCGGAGCCCAGAATAAGTGAAGGCCCGTAGCAAATTCCATGCGAAGCCAACG
> seq118
AAACTGCTAGCTACAAAATCTATCGGGCGCTGAGCTAACGCTGCACGTGTTCGTGCATTACTTGAAGTAAGCCACCCCGTCTTTACCACCATTACTCGGA
> seq119
CGGAAGGCTATGGTGTAAGACCGGATTGGCGACGGGTACATTCCACTAGTATAACACAAACGGGCGCCAAAGGTAACAGCTCCCAGTACTCTAATTTTCT
> seq120
GGTCGCCAGAAGCATCTGATAATTTAGGGACCTGGGCCAAGTCATTAGAAACGAACAAATCTTGAACGAACGCTTTGTGATCGTTGGGTCAGGCATCGGG
> seq121
TTGGGCCAAAGGCCTAACCTGCACACTTAGCAGTGGGCAGATCCTCATACTTAGAACTCTCTATGCCCGGGTAATAGAGCTACCGCCTGTCTCCCGCAAC
> seq122
GACAAGTAAATACGATATGATTGCGTCACCCTCTTGCCTTCCTTGGCCCGGCTTTGCACTGGACCTCTAGGGAATGAAATAAGATGGCTCGCTATTACCC
> seq123
GGCTCATCGTTGTGCTGTTCCCATTCGACCCTCTTGTTCTGCCTACAGTCCGTCTGGTTGGTCTCGTCGAATACCCACAGAGCGCCGGCGTTTAGAGTGT
> seq124
ACGATAAGTCCATAGTTGATTTAGATTGTAAAGTCAAGTAGACTACGGCTGCTTGTTCTCATAACTATCTCTCTCGTCAACAAGCACTCTACAATGCATT
> seq125
TGCACGGGAAGGAGATAGCGAGAACTTCCATATGGAAAACCACGATCCACCTGTGGACTCCATTGGCACTACTGGCAACATTTTCCGCTGCACATCATTT
> seq126
ACCGGGGTAGACACTAAATCACAAAGAAGCGTGTGGTTGTTTCATTGAGTTTACCCAAGATCTGGCGGACGATTGCTGGGCTTACCGTAACGTTACTGGG
> seq127
ATTGGGGCGGCACGTCGACAAAAGCCACTCCAGGGACGATCTCACGCAGAATCCTTATAGAATTTGCCATAATCCTACCATCGAACAAGCTATATGCCTT
> seq128
CCTCCATCTTCGAAGAGCCATCGACCACCTTATTTACGGAACGCACTAATGCAGCTGGGCTGAATAAGGCGTGAGTATGCACCACCACCTTGGCCTGCCC
> seq129
ACAAGCGAATCGCTTGTGACCACACCTGCTTGGCTACCACACTTACCTTCTCGCGTAGACCCTCGATAGCAAGGAGTTGGCCCTGAAAGCCAAATGTGTC
> seq130
CTAGAACCACCACTAGGCGAGTTTATGAACCGGGGTTATAGATAGGTCCCTGCAGACAGTGTAAGATACTGAATCAGTGCCCTGGCTGGACTGGGGTGGC
> seq131
ACTGTGGCGAACGAACCCTCTACGCAATTGGGTTCTGAGGAAATTCTCGGAACATATCGAGCTGTAGCGTGTGACCTACTATCACGACTGACGTACAGTT
> seq132
AAATGCGAGGGTCAATGCTTTGTGATTCAATGTCAAGCCGTTAATCTCTATGGTCACCCCGGTTACCTTAGGAATTACACCATATATCAGGAGCTTCCCA
> seq133
AAGGCCTGAGATCGTTTCGTATAGAATATGAAAATAGGTGAGGACACGAAAGATCAAGCTGAGGCGCCAAGAGTGGTAAGCGCCGGTTGTACCGCTAACA
> seq134
TGCAGCGCCGTGTTGGAAGAGACCTATTGCATAAACAGCGGGGACACCTGCCCGGGTTCTTTTTGGGATAGCACGGAGCGCTGAGCGCCAGAGCCAGCAA
> seq135
ACCTTGATACAAGACCCAGCGCTTCGACGCGCGCGCACACCAGGCCAGTTACTTCACTAGTCTACCAAGTGATCGATCGTCATACGAGTGTGTTGGGTTC
> seq136
CACAGCGGGAACCGCACTTGCCTCAAACTGGTGTAAGAAAAGCTTATACTTCGGACCCAGGAGTTTTCCAGGCGGTGGGGCAGTATCTAGGGTAGGATGG
> seq137
AAGGGTCTTAGTGGATTCGCGCGGTGGGCGTTGAGCTTACGTATAGGTTCGTTCATCTTCTTGCGGTCCAGTCGCTTCAATCGAGGACGCAATCTAGTCA
> seq138
CACCTTCTAAGAAGCCTCCTTAGCTCCTGGTACGCGGTAGTTTAGACAGAAGTGCATCAGGAGAATAACATGTAGTGATATGCTTGGGCTAGTTGAAAGC
> seq139
GGGTCTCCGATCTAGGCTGTAAACGGCTGCTCACCGCTGTTATAACGCTCCGTCTCGTCATACCAGATATTACTGGGATGTTAGTGGGGAACTACTTGAT
> seq140
GACCTTAACGTACCCCTCGTTCATACCGTTGAGCCGCCGAGTCGTGGTCGAGTATGTTTATTAAGACAACCCGCAAGAGGAGCCCTGATGCGGGAACATC
> seq141
CTTAAACTGCACATAAGAGTGGACGTACATTCGACTGTTAACAGAGAAACATACATAGTTTGTTAGGCAGCTAATTGTCGGCCTAGATTCCAGAGGTGTA
> seq142
ATGTAAGTTATATTAGCCAAAGATCAGTCTTCTACTGGAAGGTTTGCGGGGTCGATCGGTGAGTAGATCGACTTGCAACGCTCTTTACAAGTTCGATTTG
> seq143
GTAATCCCTAATTGTCCGGGGCTAACTCATATGGGCCTTTTCGAGCTGAAATCTCACATCTCAGCCACACCAAACTCCCATGTTTTACATAATCTAAACT
> seq144
CGCGGTGCCCTGACGGATATGGGGGCTACCCCGCGAGCAGCCATTAAAACATCTTATGAATAACAGAGAATTCGTACATCACAACTTAAGTACTTTTTCA
> seq145
GTCTCTGGTTCGGTCATCCGTACTCTCCGGACTGATAAACGAATGGGAGATGTGCAGTCTGCTGCAGGCGCCCTAACACCTACCGAGAAGCACATACTGA
> seq146
AATGAATCGGACTGGGGATACCAACTGTTAGGAATGTAAGCCTTGTTGAGGGGTGGATCGGAGGTAACCGAGAAAAATACCCTTATGATTGACCAAAAAC
> seq147
TTGTTCGCGTCCGATTAGTATGACTCAATTTGCGGGGCGATCATTCACCTGGTGAAGGTGGGGTTGTACAGTCTCATGACATGTAGGCCAGCAGCGCAGA
> seq148
AATGCGGCGCTGGTCCTCCTTAGTTGGCCACTCGTTATAGGGCGAGTAGAGGCTTTGATTTGTTTCGACTATGCAATTCTTCAATGTCTATACATGAGGA
> seq149
AACGTGCTCTGCACGTGTGGCAATCATTAGATGATTGGCAAAGAATGTCCATCCGTTATAAGACCGGGAAGCGAATAAAGCCAGTTAGTAAGTCCGCCGT
> seq150
ATGCGTTGGGCTGGCGTGCTTTTACATTACAGAATGAATGTTCGCTCACCACCTCCCTATCGAAGAAGAATTTGCAAATATTATCCTTACACGAGTTTTG
> seq151
CACGTGTCTACGTACGATTAGCGCAACGATATTGAAACTTAGCTATTATGTAATTGGTCTTAAAGCGCCCGCCAGCGGTTATTGGATTATGTGGGAGTCT
> seq152
TCAGCAATGGACCAACTCAAGGCGAAACAGTATTGGTTGGGTGAGGATTTTTTCTGCAAAGTACCCGCAAGTAAACCAGGGCCTACTTGGTTGTTTCATG
> seq153
GTGCGTGCATAGCTGGGTAAGTAGAGCGATACAGGCCACGGATTGGAGACCTCGAGCTTATTGAAGACTCCGGTTAGTGTCTATTTATGACCTTAAAGGC
> seq154
CGTCCTGACTCTGATCCTGGGCTGTCGTTGCATGTCTTTAGCTTGTACGTAAACTGGGACTTTACATACGTATCCATCAGCGGACAGGATAAACAGACAG
> seq155
CAGTTAAGGTTCACGACTCCCTTAACACGATTGACTTGCTTACAGGCGGCGTGGCGAGGCGTTAACCCAGGCCAGCACGGAGCCATATCAGGACCTGCGG
> seq156
CCCGACGCTATAACATTGCGCGCTACACGGGTTGAAGCCCTTCTCCCAAGTTCGTAGCCCGGATGAGTATCCATCCTAGACTAGGAAATGTTCAGGTGTT
> seq157
CCGCAAGGATTGCAAAGTCGCTCGTTGTCTAACTTGCGTCGCTACATAATCGTTGACGGTACCGACAAGGTGGATCACCCAATCAGTGCTAGTTTCTCTC
> seq158
TAATCGATCGGCAAAGTGTGTATGTTTGAACAAGAAATTTTATGAAGCTGTAGTCCCAGATCATAATCTTGAACAACATCATTATTTATTGCTTTTCACG
> seq159
ACAAGGTTTGTTCTGATCCAAAACTGTCGTAGATCGAATTAAACTAAGGGCCCTAGAGCTATGAGGACAGGAAATGTGCCGAACGGGGGAACAGCCTGAT
> seq160
CTTATAAATTGGGGTCATTAAAAGTCTGACCCACCGTAGTTTGATCGGAGTAGTCCGTTATACTGTAAATTACGCTTTCGTAAGTTACGCTCCGGTGCCA
> seq161
ATATATCAGCGCACTGTCTCCTCTCAACCTCATCCGGGAGACTACTAGCAATGGCCGAAAACCAAGAGAACGCGCTGTGCCCGGCAACACATGCATCGGG
> seq162
CCAATACTTCAACACGCCCGTGCATTCCTGCGATTTGGTCAGGTTCCGGGGGCTCACAAATTGCCCATCGTGTCGAGCACTACCGTTACTTCATACATCA
> seq163
GATGGAATAACGTGTTATATAATTTGGGGAGGATCAACGTTCTAGTTGAAAAGTAAAAGGAGCTTATCTCGGCCATGCACCACTGTATAGAATTAGCTGG
> seq164
TTTCACAGACGGCTCAGAGTGCATGTTGCCTAACCAGCATCCAGAAAGGGGAGTACACTCGTTTAGGCAAGCCCCTTAATCTACAAATTTAAACGTAGAC
> seq165
GCACCCAGTACTCACACAAGGAAGTTGGCGGCTCGATCGGGACAGAATGCAGCAATGAGTCCACGCTGTAAGTTTCTCGCTCTCCTCGACAGGGCACGAA
> seq166
CCACTCAAGTTAGAGATGCAAACGCAAGTAAAGACGTCGGTCTTTATTAGGACGGTGTCTTCGTGGTTCCTTACCTCCACTTCAAAAGAAGATGTAGTAA
> seq167
TGGCCATGTTGATGAGAGAATTCACTGATGTGTTTCTCATCTATTCCGCTAAGAACGGTAGTGCTGCTGGCGTACAAAGAGTGATAAGAGCGTTTGCTGC
> seq168
GGAACAAAACTTAATAGCTTCTCCTTGGCACGCCAAACAGTGCCACTACTAGCGTCAAATCAGCGTCTGTAGTGAGTGAGAAGAGGTAAAAGCATCGCAC
> seq169
GCCTCAATACGAAGTCACCCTCCTTTGCGCTATCTGGGTGGCCCGTGATTTAATGACTACCCACACGATGGAATAAGCCGGCGTTCTCAGTTCGTGTATT
> seq170
GTAGCTCCTAACTCTCGACAGGTACGGTTATGGTATCTTACACGCTATCTGTGTCACGGTGATCTGAAACTTTCATAGTGGGGACCTCGTGATATCGGTA
> seq171
TGATGAATCGAACTACAAGCGTCAGGCTCGTTTCCTGACGGTGTCCCGATTTAGTAGAGATACTGCAGATCAACCGCTGAGCACGCCGGCCGCATACGTC
> seq172
GTGTTGACCTGGGGATAGGACCGAGTAAGCGTCCTACCCCGGGTGATAGTTGTCTCATAAGCTTCCGTTTGGCGGCAACAAGATTAGGCCCGGCCCTGCA
> seq173
ATTTCTTTTAATGTCCTTCTCGGTCGCACGTTACGACACGAAACCCCCCCGAAAAAAAATTTAGACATGGAGCTAGTTTAGCAGGCGGCGGACACGTCAT
> seq174
GAATTGAGTCCACGGCCAGTTCTTACGCTGAAGAGCGTCTGTGGTGGTTATATGACGCTATTTGATGGTCTACTTAATACTCTTGACGCTGTGAGAACAG
> seq175
TATGACATCACATCTCTGTGTTAAGGTAGATCAATAGCTACCGCGCCCGTTGCCCCGAGCCTAAGAATACTGTTGCAGTAACGGACGTGGGCTATTAACT
> seq176
TAACTTAAATGGTAAATCATTGTACTTGAAATTCCAGCACCTTAGTCGGAGTGTAGGACCGCGCGAGTCCAAGTCACAAAGTTTGGCGCTACAGGTTTCG
> seq177
GCAAATACAAGGTAAGCAATCGAGTGATCTGTACATTTAGTCCTGTATGGATCTTAGAGTTATTTTCATATACCTGACATAATGCTTATCTATCTCCCGT
> seq178
CGGGAGTTTTTATCCGCAATAGTACGTTCTTGTGCTTTTAGCGTGAATTGTTATTGGGGGTCTTACACATCCTTACCGATAGATAGTTCCCGTTAACGAT
> seq179
GGGCGACCCCGCGCTAAACTCAACACAGCTGCAGCTACCCGACTGCAGTGATCATGCGTCATCCCGCTAAACCCGGAGCATGTTATATATTCACCAAGTG
> seq180
GTGTGCTGACGCGTGACGTTCGCAAAGGTCTAGTGATTATTATTCTTCTGATCAATTCACAGCAGACTTAGCCTTTCAGGAATCCGAAACTTGTCATTTG
> seq181
CGACATAGGACCCGTGCGGGGCGGGTATATCTCCTCCCCATAAAGTGATAACGTGACAGTGGTTCCGCCGGAACGGGCACTGCTCTCGGACAGAACTGGG
> seq182
TTATGAGAAGAGCACGCCCCTGAAATAAATTTCTCCCCAGAACAGTGAGAAAGAGGCAAACCGGTTACAGATACGATTTTATGGCCCCGAGGTCAATTAC
> seq183
GGCGTCCGCTCATTCTTTGCAGAGCGGGGCAAGGGGTGGTTAGCGTGAGGGTGTAAAGGAAACATCCCACCGGTCCACAATGTAATGCACAAGCGCATGA
> seq184
CGAGCAGCTTAACAGCATGGGGAGCAGCTTTCTTCCGGTGGGGAAAGACTCGTTTTACAAGGTTGTCCATAGCGGCCCACGTCGCATTTGCCTCCCCGAC
> seq185
TCTTTACAATGGATTGCCTTACCTTCATCCCATGGCGCACAACATATACAACAGGGCTACAACTTATCCCGGTAGCCAGTCGATAGTGCACAGAAGCTTC
> seq186
CTAACAAGCTGGCCGGGATCTGCTTTTGAGAGATTAGTAGCGGCATCCAATGGAGATATACGCATCCAGTAGTCGTAATCACCGCGGGTTCGGCTGGTAT
> seq187
CGGCCGAACGTAACCTCCCCACGCTCGACACAAGACGACGATCATCACGGACGCTGAGTCCTTTGACCGGTTGACTCTAACTGTGTACGATCCTGATAGA
> seq188
AACGAGAGTACCAATTAGCCCCGCTCCGGCATCTGGAACGATCTACACTCACCGAAATACAGGGTCTACTTGATCCCAGTTGAATGCGTAAGGTACGCGC
> seq189
CGCTGGCCTTCAACTAGCAATTAACACTAATGACTATGAATAGATTCATTGGCTCCGGCATGTTAAACGCAACTCCCTGTGTAGATGAAGACACTCTACG
> seq190
CTTGGATGACACATGGAATAGGGACTGATTCACAGCACCGCTCGGGCTATGTCAGTGAGGGAATATTGTTCACGCGGAGACGAACATCGAATCCCAGTAA
> seq191
TTCACCCAAGTGGACACTCGACTATTATGTCAGAGGGGACCACAGTTCATAGATCATGTCCACATTCGTCGGTCTCTCTTGACAACTCCTGCAACGTGTC
> seq192
CAGACCACAGTTCTGCGTGGATCGGCGTGCGTGTGCCACGAATCAAACACTTTCACGGACGTGTTTGCCCCGGTCCGTTCTTAGCAGTAGCAGCAACTGG
> seq193
AGCATACTATCAGATAATTCGCCGACTTTTTAAACTCTGTCCGAGACGATTATTTACTTTTGGTATATATGCTGTACCCCCAAAGGAGTATTCGAGATAT
> seq194
ATCGACTATCCCATTTTCGTTGCTTTGGTAAACGAATAGGCCCACCTCCTTATTCACGTCATCGACCTGCGTGGTCTCCCGGGGCGAAGCTGTGCTAATA
> seq195
ATACGTAGGGTTCACAGGTATAACCTTACATACCGCGTCTGTCTATATATTAAAACAGGGGAGACCGTTTTGACAGACGCTACTCGCGGGGGAGGAATTG
> seq196
CACACCCGTTTGCGCGAGCAGCTAACTTAAACCATCTCCCGCCAGCGTTACCGATCTCTCCACAGTGCGACATTACATGTGGACCTACGCAGTATTGGAC
> seq197
CTTGTTGAACGGCAACCAGGCTCTCTCGCAACTGACCAGCGCGGCGGCTCAGGAGCTTTTAATTTCAGGGAAACGAGGCGAGAGGCCCGGTGAATATTTC
> seq198
TCGTACACTAGGGCTGGACATTGTGATCTGGCCCGCTGTAGAGTCATCGGGACTTGCTGGAAGAACCACGATTTACGGGTGGGATCTGTGAGAACGGAGA
> seq199
GGCGCGCACCATTCCAAACTCTCCTACGGACGTCCGATGGCGCTAGGAGCGGCATAGTCCAGGGTATAGGCGATATCGGCTTGGATCCCGACACTTCACA";

    c.bench_function("reader 10", move |b| {
        b.iter(|| reader(black_box(fasta_100.as_bytes())))
    });
    c.bench_function("reader 20", move |b| {
        b.iter(|| reader(black_box(fasta_200.as_bytes())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
