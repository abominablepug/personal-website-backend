using CSV
using DataFrames
using Plots

dimensions = read("pos_config.txt", String)
df = CSV.read("pos_output.csv", DataFrame)

if dimensions == "2d"
    println("Creating 2d plot of position trajectory...")

    x_pos = df.x
    y_pos = df.y


    plot(
        x_pos,
        y_pos,
        title="Simulated Position Trajectory",
        xlabel="X Position (m)",
        ylabel="Y Position (m)",
        zlabel="Z Position (m)",
        label="Path",
        linewidth=2,
        color=:blue,
        legend=false
    )
else
    println("Creating 3d plot of position trajectory...")

    x_pos = df.x
    y_pos = df.y
    z_pos = df.z

    plot3d(
        x_pos,
        y_pos,
        z_pos,
        title="Simulated Position Trajectory",
        xlabel="X Position (m)",
        ylabel="Y Position (m)",
        zlabel="Z Position (m)",
        label="Path",
        linewidth=2,
        color=:blue,
        legend=false
    )
end

println("Saving results to trajectory_plot.png...")
savefig("trajectory_plot.png")
