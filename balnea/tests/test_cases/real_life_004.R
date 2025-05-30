tm_g_lineplot <- function(label,
                          dataname,
                          parentname = ifelse(
                            inherits(strata, "data_extract_spec"),
                            teal.transform::datanames_input(strata),
                            "ADSL"
                          ),
                          strata = teal.transform::choices_selected(
                            teal.transform::variable_choices(parentname, c("ARM", "ARMCD", "ACTARMCD")), "ARM"
                          ),
                          x = teal.transform::choices_selected(
                            teal.transform::variable_choices(dataname, "AVISIT"), "AVISIT",
                            fixed = TRUE
                          ),
                          y = teal.transform::choices_selected(
                            teal.transform::variable_choices(dataname, c("AVAL", "BASE", "CHG", "PCHG")), "AVAL"
                          ),
                          y_unit = teal.transform::choices_selected(
                            teal.transform::variable_choices(dataname, "AVALU"), "AVALU",
                            fixed = TRUE
                          ),
                          paramcd = teal.transform::choices_selected(
                            teal.transform::variable_choices(dataname, "PARAMCD"), "PARAMCD",
                            fixed = TRUE
                          ),
                          param = teal.transform::choices_selected(
                            teal.transform::value_choices(dataname, "PARAMCD", "PARAM"), "ALT"
                          ),
                          conf_level = teal.transform::choices_selected(c(0.95, 0.9, 0.8), 0.95, keep_order = TRUE),
                          interval = "mean_ci",
                          mid = "mean",
                          whiskers = c("mean_ci_lwr", "mean_ci_upr"),
                          table = c("n", "mean_sd", "median", "range"),
                          mid_type = "pl",
                          mid_point_size = c(2, 1, 5),
                          table_font_size = c(4, 2, 6),
                          plot_height = c(1000L, 200L, 4000L),
                          plot_width = NULL,
                          pre_output = NULL,
                          post_output = NULL,
                          ggplot2_args = teal.widgets::ggplot2_args()) {}
