# Boris CLI
This project leverages the resilient and accurate nature of the Boris algorithm to calculate the trajectory of a single particle acted upon by electromagnetic forces. It takes user input for initial conditions—including the uniform electric and magnetic fields—and calculates the trajectory based on those values. The Boris algorithm is the powerhouse of this project, capable of preventing numerical drift over tens of thousands of steps. This project was developed to learn how the algorithm functions and to serve as a foundation for future applications.

<p align="center">
  <img width="600" height="400" alt="trajectory_plot" src="https://github.com/user-attachments/assets/f72082e3-3b35-40c3-9e5b-8280debb82c6" />
</p>

## Implementation
The program is written using Rust for the bulk of the calculations and data storage, while Julia is used to read that data and generate a local graph. The Rust side performs all calculations and outputs a `.csv` file containing the projected trajectory position at each time step. These values are then passed to the Julia script, which reads and plots them to create a graph similar to the one shown below. The program supports both 2D and 3D plotting to provide a comprehensive view of the trajectory.

## Formulas
The Boris algorithm consists of a series of formulas that use half-steps and previous values to accurately determine the next state based on the applied electric and magnetic fields. These formulas state that:
<br>

$$
x_{k+1} = x_k + \Delta{t}v_{k+1/2}
$$

$$
v_{k+1/2} = u' + q'E_k
$$

where

$$
u' = u + (u + (u \times h)) \times s
$$

$$
u = v_{k-1/2} + q'E_k
$$

$$
h = q'B_k
$$

$$
s = \frac{2h}{1 + h^2}
$$

$$
q' = \Delta{t}\cdot\frac{q}{2m}
$$

The algorithm centers around the first two equations, which solve for the position and velocity at the next step of the simulation. The velocity is calculated at each half-step to ensure the accuracy of the simulation, as numerical drift is extremely common over thousands of iterations. To achieve this, the velocity must be stepped back by a half-step at the beginning of the program. The remaining five equations supply the necessary variables and information used within the first two equations to solve for the new velocity and position at each step.

## Results
One of the most common methods to stress-test an implementation of the Boris algorithm is to observe whether a particle, under a purely magnetic field in the z-direction, moves in a perfect circle. Running the simulation with a particle starting at $(0, 1, 0)$ with an initial velocity of $(1, 0, 0)$, a charge and mass of $1$, and applying that purely magnetic field over $10,000$ steps produced the following graph:
<p align="center">
  <img width="600" height="400" alt="trajectory_plot" src="https://github.com/user-attachments/assets/79c80f8f-e7d3-4afd-8f6c-a13675387562" />
</p>

