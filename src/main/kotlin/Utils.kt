fun <K, V> Map<K, V>.updated(key: K, transform: (V) -> V): Map<K, V> {
    val value = this[key] ?: return this
    val updatedValue = transform(value)
    return this.mapValues { (k, v) ->
        if (k == key) updatedValue else v
    }
}