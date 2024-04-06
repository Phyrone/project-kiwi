package de.phyrone.kiwi.build

import com.squareup.kotlinpoet.AnnotationSpec
import com.squareup.kotlinpoet.ClassName
import com.squareup.kotlinpoet.FileSpec
import com.squareup.kotlinpoet.KModifier
import com.squareup.kotlinpoet.PropertySpec
import com.squareup.kotlinpoet.TypeSpec
import java.io.File


object MetadataBuilder {
    fun buildMetadata(
        data: Map<String, Any>,
        outputDir: File,
    ) {
        val typeSpec = TypeSpec.objectBuilder("BuildMetadata")
            .addAnnotation(ClassName("kotlin.js", "JsExport"))
            .addAnnotation(
                AnnotationSpec.builder(ClassName("kotlin", "OptIn"))
                    .addMember("%T::class", ClassName("kotlin.js", "ExperimentalJsExport"))
                    .build()
            )
            .addData(data)
            .build()

        val fileSpec = FileSpec.builder("de.phyrone.minecraftcloud.metadata", "BuildMetadata")
            .addType(typeSpec)
            .build()

        fileSpec.writeTo(outputDir)
    }

    private fun TypeSpec.Builder.addData(data: Map<String, Any>): TypeSpec.Builder {
        for ((key, value) in data) {
            addProperty(createProperty(key, value))
        }
        return this
    }

    private data class PropertyData(
        val const: Boolean = false,
    )

    private fun createProperty(key: String, value: Any): PropertySpec {
        val type = when (value) {
            else -> value::class
        }
        val builder = PropertySpec.builder(
            key.uppercase()
                .replace("-", "_")
                .replace(".", "_")
                .replace(" ", "_"), type
        )
        when (value) {
            is String, is Number, is Boolean -> builder.addModifiers(KModifier.CONST)
            else -> {}
        }
        when (value) {
            is String -> builder.initializer("%S", value)
            is Boolean, is Number -> builder.initializer("%L", value)
            else -> error("Unsupported type: $type")
        }

        return builder.build()
    }


}