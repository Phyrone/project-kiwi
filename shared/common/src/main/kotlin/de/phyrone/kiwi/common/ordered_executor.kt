package de.phyrone.kiwi.common

import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.joinAll
import kotlinx.coroutines.launch
import kotlin.reflect.KClass
import kotlin.reflect.full.findAnnotations


enum class ExecutionOrder {
    FORWARD, BACKWARD
}

//WARNING: cyclic dependencies will cause a deadlock
//TODO add cycle detection
/**
 * Execute ordered ensures that the given list of [OrderedInvocable] will be executed in the correct order based on the [order]
 * It ensures that all dependencies are executed before the given [executor] is called on the [OrderedInvocable].
 * [OrderedInvocable] items that do not relate to each other will be executed in parallel (the entire point of this function).
 *
 * Dependencies are defined by the [DependsOn] annotation.
 * Declaring a dependency which is not in the list will be ignored.
 *
 * WARNING: cyclic dependencies will cause a deadlock and are not detected yet
 *
 * @param order the order in which the [OrderedInvocable] should be executed
 * @param executor the function that should be called on the [OrderedInvocable]
 */
@JvmOverloads
suspend fun <T : Any> List<T>.executeOrdered(
    order: ExecutionOrder = ExecutionOrder.FORWARD,
    executor: suspend (T) -> Unit,
) {
    val entities = this.map { ExecutionEntity.create(it, executor) }

    coroutineScope {
        when (order) {
            ExecutionOrder.FORWARD -> {
                entities.map {
                    launch { it.addDepndencies(entities) }
                }.joinAll()
            }

            ExecutionOrder.BACKWARD -> {
                entities.map {
                    launch { it.addDependenciesReverse(entities) }
                }.joinAll()
            }
        }


        entities.map { entity ->
            launch { entity.waitAndExecute() }
        }.joinAll()

    }

}

private class ExecutionEntity<T : Any>(
    val entity: T,
    private val executor: suspend (T) -> Unit,
) {
    val dependClasses = entity::class.findAnnotations<DependsOn>()
        .map { it.system }

    var dependencies: List<ExecutionEntity<T>> = mutableListOf()

    val state = MutableStateFlow(ExecutionState.WAITING)

    enum class ExecutionState {
        WAITING, EXECUTING, FINISHED
    }

    fun addDepndencies(list: List<ExecutionEntity<T>>) {
        dependencies = list.filter { entity -> dependClasses.any { it.isInstance(entity.entity) } }
    }


    fun addDependenciesReverse(list: List<ExecutionEntity<T>>) {
        dependencies = list.filter { entity ->
            entity.dependClasses.any { it.isInstance(this.entity) }
        }
    }

    suspend fun waitDone() {
        state.first { it == ExecutionState.FINISHED }
    }

    suspend fun waitAndExecute() {
        dependencies.forEach { it.waitDone() }
        execute()
    }


    suspend fun execute() {
        state.emit(ExecutionState.EXECUTING)
        try {
            executor(entity)
        } finally {
            state.emit(ExecutionState.FINISHED)
        }
    }


    companion object {
        //yes yes the my last months writing rust are showing up here
        fun <T : Any> create(
            entity: T,
            executor: suspend (T) -> Unit,
        ): ExecutionEntity<T> {
            return ExecutionEntity(entity, executor)
        }
    }
}

@Target(AnnotationTarget.CLASS)
@Retention(AnnotationRetention.RUNTIME)
@Repeatable
annotation class DependsOn(
    val system: KClass<*>
)
