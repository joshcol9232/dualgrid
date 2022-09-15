#include <iostream>
#include <fstream>
#include <vector>
#include <cmath>

#include "vector.h"
#include "constructionset.h"
#include "intersection.h"
#include "state.h"


std::vector<ConstructionSet<2>> symmetric_basis(size_t degree, int k_range = 3) {
  std::vector<ConstructionSet<2>> basis = {};
  basis.reserve(degree);

  double degree_fl = static_cast<double>(degree);

  for (size_t i = 0; i < degree; i++) {
    double a = static_cast<double>(i * 2) * M_PI / degree_fl;
    auto normal = Vector<2>::new_from_angle(a);
    auto b = ConstructionSet<2>(normal, 1.0/degree_fl, i, k_range);
    basis.push_back(b);
  }

  return basis;
}

#define DIMS 2

int main() {
  auto basis = symmetric_basis(5);
  std::vector<Vector<DIMS>> verts;  // Groups of 4 form rhombic tiles.

  // Find cells from each of the construction sets
  for (size_t i = 0; i < basis.size()-1; i++) {
    for (size_t j = i+1; j < basis.size(); j++) {
      std::cout << "Comparing i: " << i << " and j: " << j << std::endl;
      basis[i].get_cells_with(verts, basis[j], basis);
    }
  }

  std::cout << "Writing to file... " << verts.size() << std::endl;

  // Write to output txt
  std::ofstream outputf;
  outputf.open("/data/users/jcolclou/cpp_projects/output.txt");
  for (const auto &vert : verts)
    outputf << vert << std::endl;

  outputf.close();

  return 0;
}
