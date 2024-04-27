import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon
from matplotlib.collections import PatchCollection
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
from matplotlib import cm
import sys

def render_cells_solid_2D(
    verts,    # 2D array of [[cell1 verts...], [cell2 verts...], ...]
    ax,
    colourmap_str="viridis",
    opacity=1.0,
    edge_thickness=0.0,
    edge_colour="k",
    scale=0.8,
    centre_of_interest=None,
    axis_size=5.0,
):
    def make_polygon(cell_verts, scale):
         # copy to new array in draw-order
        verts = np.array([cell_verts[0], cell_verts[1], cell_verts[3], cell_verts[2]])
        if scale < 1.0:
            middle = np.mean(verts, axis=0)
            for v in verts:
                v -= (v - middle) * (1.0 - scale)
        
        return Polygon(verts)

    # Group by smallest internal angle. This will serve as the colour index
    INDEX_DECIMALS = 4  # Significant figures used in grouping cells together
    poly_dict = {} # Dictionary of {size index: [matplotlib polygon]}

    for startvert in range(0, len(verts), 4):
        size_ratio = abs(np.dot(verts[startvert] - verts[startvert+1], verts[startvert] - verts[startvert+2]))
        p = make_polygon([verts[startvert + i] for i in range(4)], scale)
        if size_ratio not in poly_dict:
            poly_dict[size_ratio] = [p]
        else:
            poly_dict[size_ratio].append(p)

    # Render
    clrmap = cm.get_cmap(colourmap_str)
    for size_ratio, polygons in poly_dict.items():
        colour = clrmap(size_ratio)
        shape_coll = PatchCollection(polygons, edgecolor=edge_colour, facecolor=colour, linewidth=edge_thickness, antialiased=True)
        ax.add_collection(shape_coll)

    if type(centre_of_interest) == type(None):
        # Find coi
        centre_of_interest = np.array([0.0, 0.0])

    plt.xlim(centre_of_interest[0] - axis_size, centre_of_interest[0] + axis_size)
    plt.ylim(centre_of_interest[1] - axis_size, centre_of_interest[1] + axis_size)
    plt.gca().set_aspect("equal")   # Make sure plot is in an equal aspect ratio



if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(prog="tilerender")
    parser.add_argument("input")
    parser.add_argument("-o", "--output")
    parser.add_argument("-x", "--x-lim", type=float, default=15)
    parser.add_argument("-y", "--y-lim", type=float, default=15)
    args = parser.parse_args()

    
    loaded_verts = np.loadtxt(args.input, delimiter=",")

    fig, ax = plt.subplots(1, figsize=(10, 10))
    ax.axis("equal")

    # plt.plot(loaded_verts[:,0], loaded_verts[:,1], ".")
    # plt.gca().set_aspect("equal", adjustable="box")
    render_cells_solid_2D(loaded_verts, ax)
    ax.set_xlim(-args.x_lim, args.x_lim)
    ax.set_ylim(-args.y_lim, args.y_lim)
    
    if args.output:
        plt.savefig(args.output)
    else:
        plt.show()

