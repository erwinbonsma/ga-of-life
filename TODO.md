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

- Add "Reset" GA button
- Add Generation Step GA run configuration (to only update UI every X generations)
- Track + Show CA steps per second
- Make key GA parameters configurable
    - Pop.size, tournament size, elitism [yes/no], mutation rate, recombination rate
- Make plot configurable
    - Choose two from: best fitness, avg fitness, delta evaluations, avg CA steps, 
- Extend GA plot with CA run stats
    - Add number of generations (from MyEA with MyEvaluator)
- Add plot for CA with:
    - num alive cells
    - num once-alive cells
- Look & Feel
    - Improve layout
    - Improve theming

- Reduce WASM size