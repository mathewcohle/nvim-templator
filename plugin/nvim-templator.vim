" Initialize the channel for nvim-templator
if !exists('s:templatorjobid')
	let s:templatorjobid = 0
endif

" Path to the binary
let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/release/nvim-templator'

" RPC message constants
let s:NamedTuple = 'namedtuple'
let s:Fail = 'fail'

" Entry point
function! s:init()
  call s:connect()
endfunction

" Get the Job ID and check for errors. If no errors, attach RPC handlers to
" the commands.
function! s:connect()
  let jobID = s:GetJobID()

  if 0 == jobID
    echoerr "templator: cannot start rpc process"
  elseif -1 == jobID
    echoerr "templator: rpc process is not executable"
  else
    let s:templatorjobid = jobID
    call s:AttachRPCHandlers(jobID)
  endif
endfunction

" Function reference in case of RPC start errors
function! s:OnStderr(id, data, event) dict
  echom 'stderr: ' . a:event . join(a:data, "\n")
endfunction

" Start the RPC job and return the job  (channel) ID
function! s:GetJobID()
  if s:templatorjobid == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
    return jobid
  else
    return s:templatorjobid
  endif
endfunction

" Associate commands with their RPC invocations
function! s:AttachRPCHandlers(jobID)
  command! -nargs=0 TemplateNamedTuple :call s:rpc(s:NamedTuple)
  command! -nargs=0 TemplateFail :call s:rpc(s:Fail)
endfunction

" Send an RPC message to the remote process.
function! s:rpc(rpcMessage)
	call rpcnotify(s:templatorjobid, a:rpcMessage)
endfunction

call s:init()
