/** @derive(Debug, Clone) */
class Stepper {
    accessor step: number = 1;
    label: string = '';
}

/** @derive(Decode) */
class Inherits extends Stepper {
    extra: number = 0;
}
