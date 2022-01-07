#!/bin/sh
rm -rf plot &&
mkdir plot &&
sed -e 's/ //g' -e 's/--//g' -e 's=/=,=g' -e 's/:/,/g' bench.txt >plot/bench.data
cd plot
for charset in `sed 's/,.*//' bench.data | uniq`
do
    mkdir $charset
    (
        cd $charset
        awk -F, '
        $1==charset {
            printf("%s %s\n", $3, $4) >> $2 ".data"
        }
        ' charset=$charset <../bench.data
        script=`cat <<'EOF'
            { files[++nfiles] = $0 }
            END {
                filelist = "'" (files[1]) "'"
                for(i=2; i<=nfiles; i++)
                    filelist = filelist ", '" (files[i]) "'"
                printf "plot %s\n", filelist
            }
        EOF`
        plot="`ls *.data | awk \"$script\"`"
        cat <<EOF |
            set terminal png;
            set output '../../$charset.png';
            set style data linespoints;
            $plot;
EOF
        gnuplot
    )
done
