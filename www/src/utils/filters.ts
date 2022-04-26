export class RangeFilter {
    id: String;
    label: String;
    min: Number;
    max: Number;
    value: Number;

    constructor(id: String, label: String, min: Number, max: Number, value: Number) {
        this.id = id;
        this.label = label;
        this.min = min;
        this.max = max;
        this.value = value;
    }
}

export default {
}