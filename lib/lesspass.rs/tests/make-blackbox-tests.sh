#/bin/sh

LP="$1"

if [ -z "$LP" ]; then
    echo "Usage: $0 <PATH-TO-LESSPASS-CLI>"
    exit 2
fi

TEST=0
for SITE in example.org example.net; do
    for LOGIN in user@example.org user; do
        for PASSWORD in password foobar; do
            for COUNTER in 1 8 32 100000; do
                for LENGTH in 12 16 32; do
                    echo "#[test]"

                    if [ $TEST -ge 4 ]; then
                        echo "#[cfg_attr(debug_assertions, ignore)]"
                    fi

                    echo "fn vectors_$(printf "%03d" $TEST)() {"
                    echo -n "    t($TEST, "
                    echo -n "\"$SITE\", "
                    echo -n "\"$LOGIN\", "
                    echo -n "\"$PASSWORD\", "
                    echo -n "$COUNTER, "
                    echo -n "$LENGTH, &["
                    for LOWER in no- ""; do
                        for UPPER in no- ""; do
                            for DIGITS in no- ""; do
                                for SYMBOLS in no- ""; do
                                    if [ "$LOWER$UPPER$DIGITS$SYMBOLS" \
                                             = "no-no-no-no-" ]; then
                                        continue
                                    fi
                                    echo -n "\"$($LP $SITE $LOGIN $PASSWORD \
                                                       --counter $COUNTER \
                                                       --length $LENGTH \
                                                       --${LOWER}lowercase \
                                                       --${UPPER}uppercase \
                                                       --${DIGITS}digits \
                                                       --${SYMBOLS}symbols \
                                                   | sed -e 's/\\/\\\\/g' -e 's/"/\\"/g' )\""
                                    echo -n ", "
                                done
                            done
                        done
                    done
                    echo "]);"
                    echo "}"
                    echo
                    TEST="$(($TEST + 1))"
                done
            done
        done
    done
done
