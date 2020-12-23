There is a `lib.rs` which contains all modules making the computations 
and drawings and there is a `main.rs` which runs `lib.rs`.
___
I used the functions of `process_csv.rs` module to process the given
`csv` file and to store the results of the filtered nodes whose inner
`x` distances are ***<1000***


I used the function of `filter.rs` module to filter the parsed nodes.
one time to get the nodes of inner `x` distances ***<1000*** and the other to get the `Node`s
whose inner `x` distances ***<100*** to plot them.

I used `visualize.rs` to plot the images.
___
In `main.rs`,
- I parsed the `csv`, filtered the nodes of inner `x` distances
***<1000***, then stored the result in a new `csv`.
- I filtered the nodes of inner `x` distances
  ***<100***, to plot them, then plotted 3 images for the nodes and the edges. the first images is **x vs t**,
  the second is **y vs x** and the third is **y vs x** but using different color for each ball of the fifty balls.
- I plotted the same images again but only for a portion of the previous nodes and edges.