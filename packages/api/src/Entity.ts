import { AbstractPointerBacked } from "./AbstractPointerBacked";

export class Entity extends AbstractPointerBacked {

    constructor() {
        super(construct_entity());
    }

    /**
     * Returns true if the entity has no components (values).
     */
    isEmpty(): boolean {
        return entity_is_empty(this.pointer);
    }

    /**
     * Returns the number of component (values) in the entity.
     */
    len(): number {
        return entity_len(this.pointer);
    };

}