#!/usr/bin/env bash

set -u

ROOT="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

EMPTY="$ROOT/empty.txt"
FOX="$ROOT/fox.txt"
SPIDERS="$ROOT/spiders.txt"
BUSTLE="$ROOT/the-bustle.txt"
SUNRISE="$ROOT/sunrise-summit.txt"
ALL="$EMPTY $FOX $SPIDERS $BUSTLE $SUNRISE"

for FILE in $ALL; do
    BASENAME=$(basename "$FILE")
    cat    $FILE > ${OUT_DIR}/${BASENAME}.out
    cat -n $FILE > ${OUT_DIR}/${BASENAME}.n.out
    cat -b $FILE > ${OUT_DIR}/${BASENAME}.b.out
    cat -s $FILE > ${OUT_DIR}/${BASENAME}.s.out
    cat -E $FILE > ${OUT_DIR}/${BASENAME}.E.out
    cat -ns $FILE > ${OUT_DIR}/${BASENAME}.ns.out
    cat -bs $FILE > ${OUT_DIR}/${BASENAME}.bs.out
done

cat    $ALL > $OUT_DIR/all.out
cat -n $ALL > $OUT_DIR/all.n.out
cat -b $ALL > $OUT_DIR/all.b.out
cat -s $ALL > $OUT_DIR/all.s.out
cat -E $ALL > $OUT_DIR/all.E.out
cat -ns $ALL > $OUT_DIR/all.ns.out
cat -bs $ALL > $OUT_DIR/all.bs.out

cat    < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).stdin.out
cat -n < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).n.stdin.out
cat -b < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).b.stdin.out
cat -s < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).s.stdin.out
cat -E < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).E.stdin.out

