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

export class CheckboxFilter {
    id: String;
    label: String;
    selected: Boolean;

    constructor(id: String, label: String, selected: Boolean) {
        this.id = id;
        this.label = label;
        this.selected = selected;
    }
}

export class ImagePreviewFilter {
    id: String;
    label: String;
    image: String;
    selected: Boolean;

    constructor(id: String, label: String, image: String, selected: Boolean) {
        this.id = id;
        this.label = label;
        this.image = image;
        this.selected = selected;
    }
}

export default {
}