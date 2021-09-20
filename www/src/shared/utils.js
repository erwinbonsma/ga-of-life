export function bound(value, min, max) {
    return Math.max(Math.min(max, value), min);
}