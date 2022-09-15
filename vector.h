#ifndef VECTOR2_H
#define VECTOR2_H

#include <cmath>
#include <iostream>
#include <initializer_list>

template<size_t N>
class Vector
{
public:
    Vector() {
        for (size_t i = 0; i < N; ++i)
            e[i] = 0.0;
    }

    Vector(std::initializer_list<double> const& l) {
        std::copy(l.begin(), l.end(), e);
    }

    static Vector new_from_angle(double angle);
    double norm_squared() const;
    double norm() const;
    double dot(const Vector& rhs) const;
    double cross(const Vector& rhs) const;  // 2D cross product - matrix det

    Vector operator+(const Vector& rhs) const;
    Vector operator-(const Vector& rhs) const;
    Vector operator*(double scalar) const;
    Vector operator/(double scalar) const;
    void operator+=(const Vector& rhs);
    void operator-=(const Vector& rhs);
    void operator*=(double scalar);
    void operator/=(double scalar);

    double& operator[](size_t idx) { return e[idx]; }
    const double& operator[](size_t idx) const { return e[idx]; }
private:
    double e[N];
};

template<size_t N>
std::ostream& operator<<(std::ostream& os, const Vector<N>& v);

// IMPLEMENTATIONS

template<>
Vector<2> Vector<2>::new_from_angle(double angle) {  // returns vector of length 1{
    return {std::cos(angle), std::sin(angle)};
}

template<size_t N>
double Vector<N>::norm_squared() const {
    double out = 0.0f;
    for (size_t i = 0; i < N; ++i)
        out += e[i] * e[i];

    return out;
}

template<size_t N>
double Vector<N>::norm() const {
     return std::sqrt(norm_squared());
}

template<size_t N>
double Vector<N>::dot(const Vector<N>& rhs) const {
    double out;
    for (size_t i = 0; i < N; ++i)
        out += e[i] * rhs[i];

    return out;
}

template<>
double Vector<2>::cross(const Vector<2>& rhs) const {
    return e[0] * rhs[1] - e[1] * rhs[0];
}

template<size_t N>
std::ostream& operator<<(std::ostream& os, const Vector<N>& v) {
    for (size_t i = 0; i < N-1; ++i) {
        os << v[i] << ", ";
    }
    os << v[N-1] << std::endl;

    return os;
}

template<size_t N>
Vector<N> Vector<N>::operator+(const Vector<N>& rhs) const {
    Vector<N> out;
    for (size_t i = 0; i < N; ++i)
        out[i] = e[i] + rhs[i];

    return out;
}

template<size_t N>
Vector<N> Vector<N>::operator-(const Vector<N>& rhs) const {
    Vector<N> out;
    for (size_t i = 0; i < N; ++i)
        out[i] = e[i] - rhs[i];

    return out;
}

template<size_t N>
Vector<N> Vector<N>::operator*(double scalar) const {
    Vector<N> out;
    for (size_t i = 0; i < N; ++i)
        out[i] = e[i] * scalar;

    return out;
}

template<size_t N>
Vector<N> operator*(double scalar, Vector<N> v) {
    return v * scalar;
}

template<size_t N>
Vector<N> Vector<N>::operator/(double scalar) const {
    return this * 1.0/scalar;
}

template<size_t N>
void Vector<N>::operator+=(const Vector<N>& rhs) {
    for (size_t i = 0; i < N; ++i)
        e[i] += rhs[i];
}

template<size_t N>
void Vector<N>::operator-=(const Vector<N>& rhs) {
    for (size_t i = 0; i < N; ++i)
        e[i] -= rhs[i];
}

template<size_t N>
void Vector<N>::operator*=(double scalar) {
    for (size_t i = 0; i < N; ++i)
        e[i] *= scalar;
}

template<size_t N>
void Vector<N>::operator/=(double scalar) {
    for (size_t i = 0; i < N; ++i)
        e[i] /= scalar;
}


#endif // VECTOR2_H
