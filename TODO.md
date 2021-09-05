Genetic Algorithm
-----------------

Problem specific:
- Different evaluators:
    - Max cells toggled. Sub criteria:
        - Minimum number of steps
        - Minimum start cells
        - Minimum cells at end

Cellular Automata
-----------------

- Check if int-representation impacts run speed

Web front-end
-------------

- Refactor CA to also use redux
    - Remove Seed button: Auto-seed CA when component is shown.
- Add Generation Step GA run configuration (to only update UI every X generations)
- Add plot for CA with:
    - num alive cells
    - num once-alive cells
- Look & Feel
    - Improve theming
    - Improve button layouts: uniform size, add spacing
- Add EA analysis
    - History of best individual

- Reduce WASM size