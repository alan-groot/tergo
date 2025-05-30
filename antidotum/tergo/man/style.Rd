% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/styling.R
\name{style}
\alias{style}
\title{Style a package}
\usage{
style(config_file = "tergo.toml", configuration = list(), ...)
}
\arguments{
\item{config_file}{(\code{character}) The path to the configuration file. Default \code{"tergo.toml"}.}

\item{configuration}{(\code{list}) Configuration for formatting. Default \code{list()}.}

\item{...}{additional parameters to \code{\link[=style_pkg]{style_pkg()}}}
}
\value{
No return value, called for side effects.
}
\description{
Style a package
}
\details{
Configuration is read from a file named \code{tergo.toml} in the root of the
package. The precedence of the configuration is (from the highest to lowest):
\enumerate{
\item The configuration passed to the function.
\item The configuration file.
}

To see possible configuration options, see \code{\link[=get_default_config]{get_default_config()}}.
}
\examples{
style()
style(config_file = "tergo.toml", configuration = list())

}
