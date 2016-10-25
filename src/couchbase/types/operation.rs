#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    /**
     * The default storage mode. This constant was added in version 2.6.2 for
     * the sake of maintaining a default storage mode, eliminating the need
     * for simple storage operations to explicitly define operation.
     * Behaviorally it is identical to Set
     * in that it will make the server unconditionally store the item, whether
     * it exists or not.
     */
    Upsert = 0,

    /**
     * Will cause the operation to fail if the key already exists in the
     * cluster.
     */
    Add = 1,

    /**
     * Will cause the operation to fail _unless_ the key already exists in the
     * cluster.
     */
    Replace = 2,

    /** Unconditionally store the item in the cluster */
    Set = 3,

    /**
     * Rather than setting the contents of the entire document, take the value
     * specified in value and _append_ it to the existing bytes in
     * the value.
     */
    Append = 4,

    /**
     * Like Append, but prepends the new value to the existing value.
     */
    Prepend = 5
}
