# Call signature:
# conditional_option(<OPT_NAME> <DESCRIPTION> IF <CONDITION> THEN <BOOL> ELSE <BOOL>)
function(conditional_option)

    # Parse first two unnamed args
    set(_co_OPT_NAME ${ARGV0})
    set(_opt_DESCRIPTION ${ARGV1})

    # Parse remaining args
    set(_co_OPTIONS)
    set(_co_OV_ARGS THEN ELSE)
    set(_co_MV_ARGS IF)
    cmake_parse_arguments(PARSE_ARGV 2 _co ${_co_OPTIONS} ${_co_OV_ARGS} ${_co_MV_ARGS})

    if(${_co_IF})
        set(${_co_OPT_NAME} ${_co_THEN} CACHE BOOL ${_opt_DESCRIPTION} FORCE)
    else()
        set(${_co_OPT_NAME} ${_co_ELSE} CACHE BOOL ${_opt_DESCRIPTION} FORCE)
    endif()

endfunction()
