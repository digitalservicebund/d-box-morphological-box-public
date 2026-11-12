/*
 * This project uses the C4 model for modelling software architecture.
 * Each software component developed by an individual team is a "softwaresystem".
 * 
 * Elements are tagged and color coded:
 * - "Existing" are already present
 * - "Deploy" are components we expect to introduce "off the shelf"
 * - "Develop" are components we develop from scratch or modify an existing codebase
 * - "Legacy" are components currently in place that will be obsoleted by this project
 *
 * TODO: annotate manufacturers for each component
 */

workspace "FITKO FIT-Neo" "Architecture of a Matrix-based communication channel for G2C-communication" {

    !identifiers hierarchical

    model {
        properties {
            "structurizr.groupSeparator" "/"
        }

        archetypes {
            database = container {
                tag "DB"
            }
        }

        g2x = group "FIT-Neo" {
            !include ./model/frontend.dsl
            !include ./model/backend.dsl
            !include ./model/relations.dsl
        }
    }

    views {
        !include ./views/dynamic.dsl
        !include ./styles.dsl
    }
}
