#ifndef CONSTRUCTIONSET_H
#define CONSTRUCTIONSET_H

#include <vector>
#include <cmath>
#include "vector.h"

template<size_t N>
class ConstructionSet
{
private:
    double offset;
    size_t setnum;
    int k_range;

public:
    Vector<N> normal;

    ConstructionSet(Vector<N> normal_, double offset_, size_t setnum_, int k_range_) :
        normal(normal_), offset(offset_), setnum(setnum_), k_range(k_range_)
    {};

    void get_cells_with(std::vector<Vector<N>>& verts, const ConstructionSet<N>& other, const std::vector<ConstructionSet<N>>& basis);
};

// IMPLEMENTATION


template<size_t N>
static Vector<N> realspace_from_indices(const int indices[], const std::vector<ConstructionSet<N>>& basis)
{
    Vector<N> out;
    for (size_t i = 0; i < basis.size(); i++)
        out += basis[i].normal * indices[i];

    return out;
}

template<>
void ConstructionSet<2>::get_cells_with(std::vector<Vector<2>>& verts,
                                     const ConstructionSet<2>& other,
                                     const std::vector<ConstructionSet<2>>& basis) {
    // Line equation given by n_0 x + n_1 y = offset + line index.

    // Find determinant of coefficient matrix
    double det = normal[0] * other.normal[1] - normal[1] * other.normal[0];

    if (det == 0.0) return; // No intersections between two vectors with determinant 0

    // Compare every line in each set with each other
    for (int k_0 = -k_range; k_0 < k_range+1; k_0++) {
        for (int k_1 = -other.k_range; k_1 < other.k_range+1; k_1++) {
            auto line0_pos = offset + k_0;
            auto line1_pos = other.offset + k_1;
            Vector<2> intersection = {( other.normal[1] * line0_pos - normal[1] * line1_pos )/det,
                                 ( -other.normal[0] * line0_pos + normal[0] * line1_pos )/det};

            // Get grid indices
            int indices[basis.size()];

            {
                int i = 0;
                for (const auto &e : basis) {
                    indices[i] = static_cast<int>(std::ceil( intersection.dot(e.normal) - e.offset ));
                    i++;
                }
            }
            // overwrite the two known ones
            indices[setnum] = k_0;
            indices[other.setnum] = k_1;

            // neighbouring indices can then be found by
            // += [0, 0], [0, 1], [1, 0], [1, 1] for parent sets

            Vector<2> localverts[4];
            localverts[0] = realspace_from_indices(indices, basis);
            localverts[1] = localverts[0] + basis[setnum].normal;
            localverts[2] = localverts[0] + basis[other.setnum].normal;
            localverts[3] = localverts[1] + basis[other.setnum].normal;

            for (size_t i = 0; i < 4; i++)
                verts.push_back(localverts[i]);
        }
    }
}



#endif // CONSTRUCTIONSET_H
