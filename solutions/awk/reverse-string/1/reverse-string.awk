END {
    split($0, src, "");
    asort(src, rev, "@ind_num_desc")
    join = "";
    for (i in rev)
        join = join rev[i]
    print join
}
